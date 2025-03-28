#[macro_use]
extern crate tracing;

use anyhow::anyhow;
use axum::extract::ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade};
use axum::extract::{self, FromRequestParts, OptionalFromRequestParts, State};
use axum::handler::Handler;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::routing::{get, post};
use axum::Router;
use axum_extra::extract::{cookie::Cookie, PrivateCookieJar};

use governor::clock::Clock;
use governor::{DefaultKeyedRateLimiter, Quota, RateLimiter};
use parking_lot::RwLock;
use rustc_hash::FxHashMap;
use sailfish::TemplateOnce;

use std::collections::HashMap;
use std::fmt::Write;
use std::net::SocketAddr;
use std::num::NonZeroU32;
use std::sync::atomic::{self, AtomicU32};
use std::sync::{Arc, LazyLock};

use runtime_axum::{layers, server};

// Things to fix:
// - Chat filtering
// - Chat message length limits
// - reduce rate-limits
// - make sure left-side wraps
// - delete channels after timeout
// - pin admin channels
// - Length limits!   Username, description, channels
// - Update channel list on delete
// - Auto-cleanup

// hide duplicate creates on the sidebar
// max ws msg size (tokio-tungstenite config)

mod utils;
use utils::Result;

mod css_filter;

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
struct UserId(u64);

impl OptionalFromRequestParts<ServerState> for UserId {
    type Rejection = std::convert::Infallible;
    fn from_request_parts(parts: &mut axum::http::request::Parts, state: &ServerState) ->
        impl std::future::Future<Output = Result<Option<Self>, Self::Rejection>> + Send
    {
        async move {
            let jar = PrivateCookieJar::<axum_extra::extract::cookie::Key>::from_request_parts(parts, &state.cookie_key).await?;
            let user_id = jar.get(USERID_COOKIE)
                .and_then(|u| u.value().parse::<u64>().ok());
            Ok(user_id.map(UserId))
        }
    }
}

const BASE36_ALPHABET: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut bytes = [b' '; 13];
        let mut n = self.0;
        for b in bytes.iter_mut().rev() {
            *b = BASE36_ALPHABET[(n % 36) as usize];
            n /= 36;
        }
        let str = std::str::from_utf8(&bytes).unwrap().trim();
        f.write_str(str)?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Invalid user id")]
struct UserIdParseError;

impl std::str::FromStr for UserId {
    type Err = UserIdParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut n: u64 = 0;
        for c in s.bytes() {
            let value = BASE36_ALPHABET.iter().position(|b| *b == c).ok_or(UserIdParseError)?;
            n = n.checked_mul(36).and_then(|n| n.checked_add(value as u64)).ok_or(UserIdParseError)?;
        }
        Ok(UserId(n))
    }
}

#[test]
fn test_user_id() {
    let user_id = UserId(23456);
    println!("{}", user_id);
    println!("{:?}", user_id.to_string().parse::<UserId>());
}

bitflags::bitflags! {
    #[derive(Debug, Copy, Clone)]
    struct Privileges: u32 {
        const CHANNEL_CREATE = 1 << 0;
        const CHANNEL_DELETE = 1 << 1;
        const MESSAGE_SEND = 1 << 2;
        const USER_KICK = 1 << 3;
        const USER_BAN = 1 << 4;
        const CHANNEL_MODIFY = 1 << 5;
        const CHANNEL_IMMUT = 1 << 6;
        const ANNOUNCE = 1 << 7;
        const FLAG_ADMIN = 1 << 31;
        const FLAG_MODERATOR = 1 << 30;

        const DEFAULT = Self::CHANNEL_CREATE.bits() | Self::CHANNEL_DELETE.bits() | Self::MESSAGE_SEND.bits() | Self::CHANNEL_MODIFY.bits();
        const MODERATOR = Self::DEFAULT.bits() | Self::FLAG_MODERATOR.bits() | Self::USER_KICK.bits() | Self::ANNOUNCE.bits();
        const ADMIN = Self::MODERATOR.bits() | Self::FLAG_ADMIN.bits() | Self::CHANNEL_IMMUT.bits() | Self::USER_BAN.bits();
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{}", .0)]
struct PrivilegeError(&'static str);

impl Privileges {
    fn require(self, required: Privileges, message: &'static str) -> Result<(), PrivilegeError> {
        if !self.contains(required) {
            Err(PrivilegeError(message))
        } else {
            Ok(())
        }
    }
}

struct User {
    id: UserId,
    name: String,
    style: Option<String>,
    privileges: Privileges,
    banned: Option<jiff::Zoned>,
    created: jiff::Zoned,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ChannelMode {
    Normal,
    Log,
}

struct Channel {
    id: String,
    description: String,
    hidden: bool,
    immutable: bool,
    pinned: bool,
    owner: UserId,
    admin_only: bool,
    mod_only: bool,
    mode: ChannelMode,
    sender: tokio::sync::broadcast::Sender<String>,
    active: AtomicU32,
    users: FxHashMap<UserId, u32>,
    slowmode: f32,
    limiter: Arc<DefaultKeyedRateLimiter<UserId>>,
    created: jiff::Zoned,
}

struct MainState {
    users: FxHashMap<UserId, User>,
    channels: HashMap<String, Arc<RwLock<Channel>>>,

    channel_list: RwLock<Utf8Bytes>,
    log_sender: tokio::sync::mpsc::UnboundedSender<LogEvent>,
    log_task: Option<tokio::task::JoinHandle<()>>,
}

#[derive(Debug)]
enum LogEvent {
    Log(String),
    Register(String, tokio::sync::broadcast::Sender<String>),
    Unregister(String),
    Exit,
}

impl ChannelMode {
    fn as_str(&self) -> &'static str {
        match self {
            ChannelMode::Normal => "normal",
            ChannelMode::Log => "log",
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Invalid channel mode. Valid modes: 'normal', 'log'")]
struct ChannelModeParseError;

impl std::str::FromStr for ChannelMode {
    type Err = ChannelModeParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "normal" => Ok(Self::Normal),
            "log" => Ok(Self::Log),
            _ => Err(ChannelModeParseError)
        }
    }
}

async fn logger(mut events: tokio::sync::mpsc::UnboundedReceiver<LogEvent>) {
    let mut channels: FxHashMap<String, _> = FxHashMap::default();
    while let Some(event) = events.recv().await {
        match event {
            LogEvent::Register(c, sender) => {
                channels.insert(c, sender);
            }
            LogEvent::Unregister(c) => {
                channels.remove(&c);
            }
            LogEvent::Log(s) => {
                trace!(log=s);
                for c in channels.values() {
                    c.send(s.clone()).ok();
                }
            }
            LogEvent::Exit => break,
        }
    }
}

impl MainState {
    fn new() -> Self {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        let log_task = tokio::spawn(async move {
            logger(receiver).await;
        });
        MainState {
            users: Default::default(),
            channels: Default::default(),
            channel_list: RwLock::new(Utf8Bytes::from_static("/channels")),
            log_sender: sender,
            log_task: Some(log_task),
        }
    }
    fn create_channel(&mut self, name: String, user_id: UserId, immutable: bool, mode: ChannelMode) -> Result<Arc<RwLock<Channel>>, Arc<RwLock<Channel>>> {
        let mut new = false;
        let arc = self.channels.entry(name)
            .or_insert_with_key(|k| {
                new = true;
                let slow = 1f32;
                let limiter = DefaultKeyedRateLimiter::keyed(
                    Quota::with_period(std::time::Duration::from_secs_f32(slow.clamp(0.25, 60.0))).unwrap()
                        .allow_burst(const { NonZeroU32::new(10).unwrap() })
                );
                Arc::new(RwLock::new(Channel {
                    id: k.clone(),
                    description: String::new(),
                    sender: tokio::sync::broadcast::channel(64).0,
                    active: AtomicU32::new(0),
                    users: FxHashMap::default(),
                    hidden: k.starts_with("_"),
                    slowmode: 0.05,
                    limiter: Arc::new(limiter),
                    owner: user_id,
                    immutable,
                    pinned: false,
                    admin_only: false,
                    mod_only: false,
                    mode,
                    created: jiff::Zoned::now(),
                }))
            })
            .clone();

        if new {
            self.update_list();
            Ok(arc)
        } else {
            Err(arc)
        }
    }
    fn ensure_channel(&mut self, name: String, user_id: UserId, immutable: bool, mode: ChannelMode) -> Arc<RwLock<Channel>> {
        match self.create_channel(name, user_id, immutable, mode) {
            Ok(c) => c,
            Err(c) => c,
        }
    }
    fn update_list(&mut self) {
        let mut channels = self.channels.iter()
            .filter(|(_, v)| !v.read().hidden)
            .map(|(s, _)| &**s)
            .collect::<Vec<_>>();
        channels.sort();
        let channels = channels.into_iter().fold("/channels".into(), |a: String, b| a + " " + b);
        *self.channel_list.write() = channels.into();
    }
    fn delete_channel(&mut self, name: String) {
        let changed = self.channels.remove(&name).is_some();
        if changed {
            self.update_list();
        }
    }
    fn get_general(this: &RwLock<Self>) -> Arc<RwLock<Channel>> {
        let id = "general".to_owned();
        let state = this.read();
        match state.get_channel(&id) {
            Some(channel) => channel.clone(),
            None => {
                drop(state);
                let mut state = this.write();
                state.ensure_channel(id.clone(), UserId(0), true, ChannelMode::Normal);
                let chan = state.get_channel(&id).unwrap().clone();
                drop(state);
                chan.write().users.insert(UserId(0), 1);
                chan.write().pinned = true;
                chan
            }
        }
    }
    fn get_channel(&self, name: &str) -> Option<&Arc<RwLock<Channel>>> {
        self.channels.get(name)
    }
    fn list_channels(&self) -> Utf8Bytes {
        self.channel_list.read().clone()
    }
    fn is_banned(&self, user: UserId) -> bool {
        self.users.get(&user).map(|u| u.banned.is_some()).unwrap_or(false)
    }
    fn privileges(&self, user: UserId) -> Privileges {
        self.users.get(&user).map(|u| u.privileges)
            .unwrap_or(Privileges::empty())
    }
    fn update_log_channel(logger: &Logger, channel_id: String, channel: Option<Arc<RwLock<Channel>>>) -> Result<(), ()> {
        let message;
        if let Some(state) = channel {
            message = LogEvent::Register(channel_id, state.read().sender.clone());
        } else {
            message = LogEvent::Unregister(channel_id);
        }
        logger.log_sender.send(message).map_err(|_| ())
    }
    async fn cleanup(this: &RwLock<Self>) {
        let task = {
            let mut guard = this.write();
            guard.log_sender.send(LogEvent::Exit).ok();
            guard.log_task.take()
        };
        if let Some(task) = task {
            task.await.ok();
        }
    }
}

struct Logger {
    log_sender: tokio::sync::mpsc::UnboundedSender<LogEvent>,
}

impl Logger {
    fn log(&self, msg: String) {
        self.log_sender.send(LogEvent::Log(msg)).unwrap();
    }
}

pub struct AppState {
    root_channel: tokio::sync::broadcast::Sender<String>,
    state: RwLock<MainState>,
    logger: Logger,
}

type ServerState = runtime_axum::ServerState<AppState>;

pub async fn start_webserver(
    bind: SocketAddr,
    cancel: tokio_util::sync::CancellationToken,
) -> Result<(), anyhow::Error> {
    let state = MainState::new();
    let app = Arc::new(AppState {
        root_channel: tokio::sync::broadcast::channel(64).0,
        logger: Logger { log_sender: state.log_sender.clone() },
        state: RwLock::new(state),
    });

    app.state.write().users.insert(UserId(0), User {
        id: UserId(0),
        name: "admin".into(),
        privileges: Privileges::ADMIN,
        banned: None,
        created: jiff::Zoned::now().first_of_month().unwrap().start_of_day().unwrap(),
        style: Some("& .username { background: red; color: white; font-weight: bold; }".into()),
    });

    MainState::get_general(&app.state);

    let flag = std::env::var("FLAG").unwrap_or_else(|_| "utflag{temp}".into());

    app.state.write().ensure_channel("log".into(), UserId(0), true, ChannelMode::Log);
    let chan = app.state.read().get_channel("log").unwrap().clone();
    chan.write().users.insert(UserId(0), 1);
    chan.write().admin_only = true;
    chan.write().pinned = true;
    MainState::update_log_channel(&app.logger, "log".into(), Some(chan)).unwrap();

    app.state.write().ensure_channel("mod-info".into(), UserId(0), true, ChannelMode::Normal);
    let chan = app.state.read().get_channel("mod-info").unwrap().clone();
    chan.write().users.insert(UserId(0), 1);
    chan.write().mod_only = true;
    chan.write().pinned = true;
    chan.write().description = format!("Congradulations on becoming a moderator!  The flag is {flag}.");

    let moderator = "moderator".parse::<UserId>().unwrap();
    app.state.write().users.insert(moderator, User {
        id: moderator,
        name: "moderator".into(),
        privileges: Privileges::MODERATOR,
        banned: None,
        created: jiff::Zoned::now().first_of_month().unwrap().start_of_day().unwrap(),
        style: Some("& .username { background: #3AE; color: white; font-weight: bold; }".into()),
    });

    let app_ref = Arc::clone(&app);
    tokio::spawn(async move {
        let mut fake_socket = SocketWrapper::Log;
        let app = app_ref;
        let mut username = "moderator".to_owned();
        let user_id = moderator;

        let general = MainState::get_general(&app.state);
        let mut cur_channel = ActiveChannel::new(general, user_id);

        enum Op {
            Cmd(Box<dyn Fn() -> String + Send + Sync>),
            Wait(std::time::Duration),
        }

        let ops = [
            Op::Wait(std::time::Duration::from_secs_f32(1.5)),
            Op::Cmd(Box::new(|| "/login unbroken-sandpit-scant-unmixable".into())),
            Op::Wait(std::time::Duration::from_secs_f32(1.5)),
            Op::Cmd(Box::new(|| {
                let now = jiff::Zoned::now().in_tz("America/Chicago").unwrap();
                let time = now.time().round(jiff::Unit::Second).unwrap();
                format!("/announce Announcement: the current time is {} (CT)", time)
            })),
            Op::Wait(std::time::Duration::from_secs_f32(5.0)),
        ];

        let start_delay = 3.0;

        let now = jiff::Zoned::now();
        let start = now.round(jiff::ZonedRound::new()
            .smallest(jiff::Unit::Minute)
            .mode(jiff::RoundMode::Ceil)
        ).unwrap()
        .saturating_sub(jiff::SignedDuration::from_secs_f32(start_delay));
        let start_delay = now.duration_until(&start).abs().try_into().unwrap();

        tokio::time::sleep(start_delay).await;

        let interval = std::time::Duration::from_secs(120);
        let mut interval = tokio::time::interval(interval);
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

        let mut idx = 0;

        loop {
            interval.tick().await;
            idx += 1;

            send_join(&cur_channel, &mut SocketWrapper::Null, user_id, &username, &app).await;

            for op in &ops {
                match op {
                    Op::Wait(duration) => {
                        tokio::time::sleep(*duration).await;
                    },
                    Op::Cmd(cmd) => {
                        let msg = cmd();
                        handle_message(&mut fake_socket, msg.into(), &app, user_id, &mut username, &mut cur_channel).await;
                    },
                }
            }

            if idx % 2 == 0 {
                handle_message(&mut fake_socket, "/msg Cleaning up old channels...".into(), &app, user_id, &mut username, &mut cur_channel).await;
                let mut state = app.state.write();
                let len = state.channels.len();
                let channels = std::mem::take(&mut state.channels);
                state.channels = channels.into_iter().filter(|(_, c)| {
                    let chan = c.read();
                    chan.active.load(atomic::Ordering::Relaxed) > 0 || chan.pinned
                }).collect();
                if len != state.channels.len() {
                    state.update_list();
                    let channels = state.list_channels();
                    app.root_channel.send(format!("/{}", channels)).ok();
                }
            }

            send_leave(&cur_channel, user_id, &username, &app).await;
        }
    });

    let app_ref = Arc::clone(&app);

    // TODO: preserve keys on disk
    let key = axum_extra::extract::cookie::Key::generate();
    let state = ServerState::new(key, app);

    let app = Router::new()
        // TODO: serve assets from memory?
        .nest_service(
            "/assets",
            layers::make_assets_router("assets".as_ref(), get(get_404)).with_state(()),
        )
        .fallback_service(main_api(state))
        .layer(layers::cross_origin_layer())
        .layer(tower_http::catch_panic::CatchPanicLayer::new())
        .layer(layers::make_trace_layer());

    info!("web server listening on {}", bind);

    server::run_server(cancel, bind, app).await?;

    info!("webserver exiting");

    MainState::cleanup(&app_ref.state).await;
    info!("webserver cleanup done");

    Ok(())
}

async fn get_404(uri: extract::OriginalUri) -> (StatusCode, String) {
    (
        StatusCode::NOT_FOUND,
        format!("path {:?} not found", uri.path()),
    )
}

fn main_api(state: ServerState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/socket", get(handler))
        .fallback(get_404)
        .with_state(state)
}

#[derive(TemplateOnce)]
#[template(path = "main.stpl")]
pub struct MainTemplate<'a> {
    pub username: Option<String>,
    pub state: &'a ServerState,
}

#[tracing::instrument(skip(state))]
#[axum::debug_handler]
async fn root(state: State<ServerState>, user: Option<UserId>) -> Result<impl IntoResponse> {
    let username = match user {
        None => None,
        Some(u) => state.app.state.read().users.get(&u)
            .map(|u| u.name.clone()),
    };

    let content = MainTemplate {
        state: &state,
        username,
    };
    Ok(axum::response::Html(content.render_once()?))
}

#[derive(serde::Deserialize, Debug)]
struct LoginForm {
    username: String,
}

const USERID_COOKIE: &str = "web-chat-userid";

fn initial_style(user: UserId) -> Option<String> {
    let idx = (user.0 % 8) + 1;
    Some(format!("& .username {{ color: var(--palette-{idx}); &::before, &::after {{ color: var(--fg) }} }}"))
}

fn login_as(
    state: &ServerState,
    mut jar: PrivateCookieJar,
    username: String,
) -> Result<(UserId, PrivateCookieJar), String> {
    use rand::Rng;

    if !valid_username(&username) {
        return Err(format!("Invalid username"));
    }

    let id = UserId(rand::thread_rng().gen::<u64>());

    let mut game_state = state.app.state.write();
    game_state.users.insert(id, User {
        id,
        name: username.clone(),
        style: initial_style(id),
        created: jiff::Zoned::now(),
        banned: None,
        privileges: Privileges::DEFAULT,
    });
    info!(id=?id, name=username, "Logged in");

    let mut cookie = Cookie::new(USERID_COOKIE, id.0.to_string());
    cookie.set_max_age(Some(time::Duration::hours(12)));
    // cookie.set_same_site(axum_extra::extract::cookie::SameSite::None);
    jar = jar.add(cookie);
    Ok((id, jar))
}

#[tracing::instrument(skip(state, jar))]
#[axum::debug_handler]
async fn login(
    state: State<ServerState>,
    jar: PrivateCookieJar,
    existing_account: Option<UserId>,
    extract::Form(form): extract::Form<LoginForm>,
) -> impl IntoResponse {
    if let Some(user) = existing_account {
        let banned = state.app.state.read().users.get(&user).map(|u| u.banned.is_some()).unwrap_or(false);
        if banned {
            return Err(Err(StatusCode::FORBIDDEN));
        }
    }

    match login_as(&state.0, jar, form.username) {
        Ok((_id, jar)) => Ok((jar, Redirect::to("."))),
        Err(_e) => Err(Ok(Redirect::to("."))),
    }
}

#[tracing::instrument(skip(state, jar))]
async fn logout(state: State<ServerState>, existing_account: Option<UserId>, mut jar: PrivateCookieJar) -> impl IntoResponse {
    if let Some(user) = existing_account {
        if state.app.state.read().is_banned(user) {
            return Err(Redirect::to("."));
        }
    }
    jar = jar.remove(Cookie::from(USERID_COOKIE));
    Ok((jar, Redirect::to(".")))
}

async fn handler(
    ws: WebSocketUpgrade,
    State(state): State<ServerState>,
    user_id: Option<UserId>,
    _jar: PrivateCookieJar,
) -> impl IntoResponse {
    match user_id {
        Some(id) => {
            let resp = ws.max_message_size(1 << 18)
                .on_upgrade(move |socket| handle_socket(socket, state.app, id));
            Ok(resp)
        },
        None => {
            // let (id, jar) = login_as(&state, jar, "anonymous".into());
            // let resp = ws.on_upgrade(move |socket| handle_socket(socket, state.app, id));
            // Err((jar, resp))
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

struct ActiveChannel {
    id: String,
    sender: tokio::sync::broadcast::Sender<String>,
    receiver: tokio::sync::broadcast::Receiver<String>,
    limiter: Arc<DefaultKeyedRateLimiter<UserId>>,

    channel: Arc<RwLock<Channel>>,
    user_id: UserId,
}

impl ActiveChannel {
    fn new(channel: Arc<RwLock<Channel>>, user_id: UserId) -> Self {
        let mut guard = channel.write();
        let id = guard.id.clone();
        let sender = guard.sender.clone();
        guard.active.fetch_add(1, atomic::Ordering::Relaxed);
        let limiter = guard.limiter.clone();
        *guard.users.entry(user_id).or_default() += 1;
        drop(guard);
        ActiveChannel {
            id,
            receiver: sender.subscribe(),
            sender,
            limiter,
            channel,
            user_id,
        }
    }
    fn reload(&mut self) {
        self.limiter = self.channel.read().limiter.clone();
    }
    fn set_limiter(&mut self, slow: f32) {
        let new_limiter = DefaultKeyedRateLimiter::keyed(
            Quota::with_period(std::time::Duration::from_secs_f32(slow.clamp(0.25, 60.0))).unwrap()
                .allow_burst(const { NonZeroU32::new(10).unwrap() })
        );
        {
            let mut channel = self.channel.write();
            channel.slowmode = slow;
            channel.limiter = Arc::new(new_limiter);
        }
        self.sender.send("/reload".into()).ok();
    }
}

impl Drop for ActiveChannel {
    fn drop(&mut self) {
        let mut guard = self.channel.write();
        guard.active.fetch_sub(1, atomic::Ordering::Relaxed);
        match guard.users.entry(self.user_id) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                *entry.get_mut() -= 1;
                if *entry.get() == 0 {
                    entry.remove();
                }
            },
            std::collections::hash_map::Entry::Vacant(_) => (),
        }
        drop(guard);
    }
}

fn user_style_cmd(state: &MainState, user_id: UserId) -> Option<String> {
    if let Some(user) = state.users.get(&user_id) {
        let style = &user.style;
        let serialized = serde_json::to_string(&style).unwrap_or_else(|_| "null".into());
        Some(format!("/style\n{{\"user\":\"{}\",\"style\":{}}}", user_id, serialized))
    } else {
        None
    }
}

async fn send_join(cur_channel: &ActiveChannel, socket: &mut SocketWrapper<'_>, cur_user: UserId, username: &String, app: &AppState) {
    use std::io::Write;
    app.logger.log(format!("/log {} {} /join {} {}", cur_user, cur_channel.id, cur_user, username));
    cur_channel.sender.send(format!("/join {} {}", cur_user, username)).ok();

    let cur_style;
    let mut existing_style = Vec::new();
    existing_style.extend_from_slice(b"/style\n");
    {
        let channel_users = cur_channel.channel.read().users.keys().copied().collect::<Vec<_>>();

        let state = app.state.read();
        cur_style = user_style_cmd(&state, cur_user);
        for user in channel_users {
            if let Some(style) = state.users.get(&user).and_then(|s| s.style.as_ref()) {
                write!(&mut existing_style, "{{\"user\":\"{}\",\"style\":", user).ok();
                serde_json::to_writer(&mut existing_style, &style).unwrap();
                existing_style.extend_from_slice(b"}\n");
            }
        }
    }
    if let Some(cur_style) = cur_style {
        cur_channel.sender.send(cur_style).ok();
    }
    if let Ok(bytes) = Utf8Bytes::try_from(existing_style) {
        socket.send(bytes).await;
    }
}

async fn send_leave(cur_channel: &ActiveChannel, user_id: UserId, username: &String, app: &AppState) {
    app.logger.log(format!("/log {} {} /leave {} {}", user_id, cur_channel.id, user_id, username));
    cur_channel.sender.send(format!("/leave {} {}", user_id, username)).ok();
}

async fn handle_socket(mut socket: WebSocket, app: Arc<AppState>, user_id: UserId) {
    let mut broadcast = app.root_channel.subscribe();
    let mut cur_channel = ActiveChannel::new(MainState::get_general(&app.state), user_id);
    let mut username = String::new();
    let err = (async || {
        if let Some(user) = app.state.read().users.get(&user_id) {
            if user.banned.is_some() {
                return Err("/error User is banned");
            }
            username = user.name.clone();
        } else {
            return Err("/error Invalid user account");
        }
        Ok(())
    })().await;
    if let Err(e) = err {
        let _ = socket.send(Message::Text(
            Utf8Bytes::from_static(e),
        )).await;
        return;
    }

    send_join(&cur_channel, &mut SocketWrapper::Socket(&mut socket), user_id, &username, &*app).await;

    enum Action {
        Skip,
        Forward,
        Close,
    }

    let handle_broadcast = async |msg: &str| {
        let (cmd, args) = msg.split_once(' ').unwrap_or((&msg, ""));
        match cmd {
            "/kick" | "/ban" => {
                let (target, _) = args.split_once(" ").unwrap_or((args, ""));
                let id = target.parse::<UserId>().ok();
                if id == Some(user_id) {
                    info!("Kicked user {user_id}");
                    return Action::Close;
                }
                Action::Skip
            },
            _ => Action::Forward,
        }
    };

    let limiter = RateLimiter::direct(
        Quota::with_period(std::time::Duration::from_secs_f32(1.0)).unwrap()
            .allow_burst(const { NonZeroU32::new(10).unwrap() })
    );

    let mut rate_limit_failures = 0;

    loop {
        tokio::select! {
            msg = socket.recv() => {
                let Some(Ok(msg)) = msg else { break };
                if limiter.check().is_err() {
                    let _ = socket.send(Message::Text(
                        Utf8Bytes::from_static("/system rate limit exceeded"),
                    )).await;
                    rate_limit_failures += 1;
                    if rate_limit_failures > 10 {
                        break;
                    }
                } else {
                    handle_message(&mut SocketWrapper::Socket(&mut socket), msg, &app, user_id, &mut username, &mut cur_channel).await;
                }
            }
            msg = broadcast.recv() => {
                let Ok(msg) = msg else { break };
                let act = handle_broadcast(&msg).await;
                match act {
                    Action::Close => break,
                    Action::Skip => (),
                    Action::Forward => {
                        socket.send(Message::Text(msg.into())).await.ok();
                    }
                }
            }
            msg = cur_channel.receiver.recv() => {
                let Ok(msg) = msg else {
                    let _ = socket.send(Message::Text(
                        Utf8Bytes::from_static("/system Switched to channel 'general'"),
                    )).await;
                    cur_channel = ActiveChannel::new(MainState::get_general(&app.state), user_id);
                    continue;
                };
                if msg == "/reload" {
                    cur_channel.reload();
                } else {
                    socket.send(Message::Text(msg.into())).await.ok();
                }
            }
        }
    }

    send_leave(&cur_channel, user_id, &username, &app).await;

    let _ = socket.send(Message::Text(
        Utf8Bytes::from_static("/error Connection closed."),
    )).await;
}

fn valid_username(name: &str) -> bool {
    !name.is_empty()
        && name.len() < 256
        && name.chars().all(|c| !c.is_ascii_control() && c != ' ')
        && !name.contains("flag")
}
fn valid_channel_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() < 64
        && name.chars().all(|c| c.is_ascii() && !c.is_ascii_control() && !c.is_ascii_whitespace())
        && !name.contains("flag")
}
fn valid_description(desc: &str) -> bool {
    allow_message(desc)
}
fn allow_message(msg: &str) -> bool {
    !msg.contains("flag") && msg.len() < 512
}

// Solve script:
// for (let msg of ["/create _temp", "/join _temp", "/set channel.admin-only true", "/set channel.mode log", "/set channel.admin-only false"]) socket.send(msg)

// Custom things
// /set user.style filter: drop-shadow(rgb(85, 255, 0) 1px 1px 0px);
// /set user.style filter: drop-shadow(rgb(85, 255, 0) 1px 1px 0px); & .username::before{content:"";}& .username::after{content:"";}

// /set user.style &::before{content:"~*~*~"}&::after{content:"~*~*~";}text-align:center;& .username::before,& .username::after{content:"";}font-style:italic;
// /set user.style & .username { text-shadow: 1px 1px red, -1px -1px #4F0, 1px -1px #33E; }

enum SocketWrapper<'a> {
    Socket(&'a mut WebSocket),
    Log,
    Null,
}
impl SocketWrapper<'_> {
    async fn send(&mut self, msg: Utf8Bytes) {
        match self {
            SocketWrapper::Socket(s) => {
                s.send(Message::Text(msg)).await.ok();
            },
            SocketWrapper::Log => info!("{}", &*msg),
            SocketWrapper::Null => (),
        }
    }
}

async fn handle_message(
    socket: &mut SocketWrapper<'_>,
    msg: Message,
    app: &AppState,
    user_id: UserId,
    username: &mut String,
    cur_channel: &mut ActiveChannel,
) {
    let Message::Text(data) = msg else { return };
    let (cmd, args) = data.split_once(' ').unwrap_or((&data, ""));

    // TODOs:
    // - Get current room status (num users, user list, channel name)
    // - Channel descriptions?
    // - Channel settings?
    // - Make sure leaked channels (active but deleted) are usable
    // - Create a custom "log" channel of all commands?
    // - Add some undocumented commands, with javascript handling
    //   hinting at their usage
    // - Figure out where to put the flag...
    // - Add some bots as a distraction?
    //    - automod, showing the kick ability?

    // Auth:
    // - "private" channels hidden via the ui, requiring privileges
    //     - Set up a "flag" channel, requiring custom admin privileges...
    //     - but how would the flag be stored?  (No history...)

    // TODO: logger bot?
    // TODO: private channel where the flag gets sent occasionally (authserv-equiv), gets logged
    // TODO: setting up custom channels as logger targets?

    // Vulns:
    // - user enumeration?
    // - Bad auth checking?  Nick & password instead of userid & password?

    // Hide channels starting with '_' from the main list; how to show
    // this to users?

    // TODO: chat filters for flag, utflag
    // -> auto warn bot?

    // TODO: prevent griefing (auto-kick bots, auto-delete bots)
    // server-wide conf for admins?  (ie. timeouts, rate-limits, default permissions)
    // expose ip -> account mappings for ip bans
    // set up ip bans?
    // proper logging of ws messages (or at min, commands)

    // TODO: custom decorations and colors

    // auto "check-in" every 5 min or so, to cause flag to be logged
    // "channel mode" - log channel, must be hidden
    // visible but unjoinable "flag" channel, w/ flag in the description
    // visible "login" command (for mod privileges, not admin)
    //  --> how do we show that moderators log in occasionally? (frequently?)
    // once the password is found, log in as a moderator and join the flag channel
    // look at the channel desc for the flag

    if !cmd.starts_with("/ctf-internal") {
        app.logger.log(format!("/log {} {} {}", user_id, cur_channel.id, data));
    }

    let res = async {
        match cmd {
            "/help" => {
                let helptext = r###"/system Help:
/help               display this message
/msg [text]         send a message on the current channel
/nick [name]        change your username

/list               list available channels
/join [channel]     switch to a different channel
/channel            show info about the current channel
/users              list users in the current channel
/user [id]          show info about the given user

/create [channel]   create a new channel
/delete [channel]   delete a channel
/set [prop] [value] configure channels or users

/announce [msg]     send a message to all channels
/kick [id]          kick a user
/ban [id]           ban a user

/login [password]   log in as a moderator or admin.  (CTF note: do not brute force.)
"###;
                socket.send(extract::ws::Utf8Bytes::from_static(helptext)).await;
            },
            "/msg" => {
                if let Err(e) = cur_channel.limiter.check_key(&user_id) {
                    let wait = e.wait_time_from(cur_channel.limiter.clock().now());
                    socket.send(format!("/error slow mode is enabled (retry in {:.2}s)", wait.as_secs_f32()).into()).await;
                } else if allow_message(args) {
                    cur_channel.sender.send(format!("/msg {} {} {}", user_id, username, args)).ok();
                }
            },
            "/nick" => {
                if !valid_username(args) {
                    return Err(anyhow!("Invalid username"));
                }
                let new_username = args.to_owned();
                let new_username_alt = new_username.clone();
                app.state.write().users.get_mut(&user_id).map(|u| u.name = new_username);
                cur_channel.sender.send(format!("/nick {} {}", user_id, new_username_alt)).ok();
                *username = new_username_alt;
            },
            "/announce" => {
                app.state.read().privileges(user_id).require(Privileges::ANNOUNCE, "You do not have permission to send announcements.")?;
                socket.send("Sending announcement to all channels.".into()).await;
                if let Some(style) = user_style_cmd(&app.state.read(), user_id) {
                    app.root_channel.send(style).ok();
                }
                if allow_message(args) {
                    app.root_channel.send(format!("/announce {} {} {}", user_id, username, args)).ok();
                }
            },
            "/kick" => {
                let user;
                if args.trim().is_empty() {
                    user = user_id;
                } else {
                    user = args.trim().parse::<UserId>()?;
                }
                let (cur_privs, target_privs) = { let s = app.state.read(); (s.privileges(user_id), s.privileges(user)) };
                if cur_privs.contains(Privileges::USER_KICK) || user == user_id {
                    if target_privs.contains(Privileges::ADMIN) && user != user_id {
                        socket.send(format!("You do not have permission to kick an admin.").into()).await;
                    } else {
                        info!("Kicking user {user}");
                        socket.send(format!("Kicking user {user}").into()).await;
                        app.root_channel.send(format!("/kick {} {}", user, user_id)).ok();
                        cur_channel.sender.send(format!("/kick {} {}", user, user_id)).ok();
                    }
                } else {
                    socket.send(format!("You do not have permission to kick users.").into()).await;
                }
            },
            "/ban" => {
                let user;
                if args.trim().is_empty() {
                    user = user_id;
                } else {
                    user = args.trim().parse::<UserId>()?;
                }
                let (cur_privs, target_privs) = { let s = app.state.read(); (s.privileges(user_id), s.privileges(user)) };
                if cur_privs.contains(Privileges::USER_BAN) || user == user_id {
                    if target_privs.contains(Privileges::ADMIN) && user != user_id {
                        socket.send(format!("You do not have permission to kick an admin.").into()).await;
                    } else {
                        info!("Banning user {user}");
                        socket.send(format!("Banning user {user}").into()).await;
                        if let Some(user) = app.state.write().users.get_mut(&user) {
                            user.banned = Some(jiff::Zoned::now());
                        }
                        app.root_channel.send(format!("/ban {} {}", user, user_id)).ok();
                        cur_channel.sender.send(format!("/ban {} {}", user, user_id)).ok();
                    }
                } else {
                    socket.send(format!("You do not have permission to ban users.").into()).await;
                }
            },
            "/unban" => {
                let user;
                if args.trim().is_empty() {
                    user = user_id;
                } else {
                    user = args.trim().parse::<UserId>()?;
                }
                let privs = app.state.read().privileges(user_id);
                if privs.contains(Privileges::USER_BAN) || user == user_id {
                    info!("Unbanning user {user}");
                    socket.send(format!("Unbanning user {user}").into()).await;
                    if let Some(user) = app.state.write().users.get_mut(&user) {
                        user.banned = None;
                    }
                } else {
                    socket.send(format!("You do not have permission to unban users.").into()).await;
                }
            },
            "/join" => {
                let privs = app.state.read().privileges(user_id);
                let new_channel = app.state.read().get_channel(args).cloned();
                let new_channel = new_channel.ok_or_else(|| anyhow!("Channel '{}' does not exist", args))?;

                {
                    let info = new_channel.read();
                    if info.admin_only && !privs.contains(Privileges::FLAG_ADMIN) {
                        return Err(anyhow!("Channel '{}' is admin-only.", args));
                    }
                    if info.mod_only && !privs.intersects(Privileges::FLAG_MODERATOR | Privileges::FLAG_ADMIN) {
                        return Err(anyhow!("Channel '{}' is admin-only.", args));
                    }
                }
                let channel = ActiveChannel::new(new_channel.clone(), user_id);

                cur_channel.sender.send(format!("/leave {} {}", user_id, username)).ok();
                *cur_channel = channel;
                send_join(&cur_channel, socket, user_id, &username, &*app).await;

                socket.send(format!("Switched to channel '{}'", args).into()).await;
            },
            "/list" => {
                let channels = app.state.read().list_channels();
                socket.send(channels).await;
            },
            "/create" => {
                if !valid_channel_name(args) {
                    return Err(anyhow!("Invalid channel name"));
                }
                app.state.read().privileges(user_id)
                    .require(Privileges::CHANNEL_CREATE, "You do not have permission to create channels")?;
                let ch = app.state.write().create_channel(args.into(), user_id, false, ChannelMode::Normal);
                match ch {
                    Ok(_) => {
                        socket.send(format!("Created channel '{}'", args).into()).await;
                        app.root_channel.send(format!("/create {} {} {}", args, user_id, username)).ok();
                    },
                    Err(_) => {
                        socket.send(format!("Channel '{}' aleady exists.", args).into()).await;
                    }
                }
            },
            "/delete" => {
                if !valid_channel_name(args) {
                    return Err(anyhow!("Invalid channel name"));
                }
                app.state.read().privileges(user_id)
                    .require(Privileges::CHANNEL_DELETE, "You do not have permission to delete channels")?;
                let name = args.into();
                {
                    let mut guard = app.state.write();
                    if let Some(c) = guard.channels.get(&name) {
                        let channel = c.read();
                        if channel.owner == user_id {

                        } else if channel.immutable {
                            return Err(anyhow!("Channel is immutable"));
                        } else if jiff::Zoned::now().duration_since(&channel.created) < jiff::SignedDuration::from_secs(60) {
                            return Err(anyhow!("Channel is too new"));
                        }
                    } else {
                        return Err(anyhow!("Channel does not exist"));
                    }
                    guard.delete_channel(name);
                }
                socket.send(format!("Deleted channel '{}'", args).into()).await;
                app.root_channel.send(format!("/delete {} {} {}", args, user_id, username)).ok();
            },
            "/set" => {
                let (prop, value) = args.trim().split_once(" ").unwrap_or((args, ""));
                let immutable = cur_channel.channel.read().immutable;
                if immutable && prop.starts_with("channel.") && prop != "channel.immutable" {
                    return Err(anyhow!("Cannot change properties of an immutable channel"))
                }
                let privileges = app.state.read().privileges(user_id);
                privileges.require(Privileges::CHANNEL_MODIFY, "You do not have permission to modify channels")?;
                match prop {
                    "" => {
                        socket.send(Utf8Bytes::from_static("Available property groups: channel, user")).await;
                        return Ok(());
                    }
                    "channel" => {
                        socket.send(Utf8Bytes::from_static("Available channel properties: .description, .slowmode, .hidden, .immutable, .owner, .admin-only, .mode")).await;
                        return Ok(());
                    }
                    "channel.description" => {
                        if valid_description(value) {
                            cur_channel.channel.write().description = value.into()
                        }
                    },
                    "channel.immutable" => {
                        let mut channel = cur_channel.channel.write();
                        if channel.owner != user_id && !privileges.contains(Privileges::CHANNEL_IMMUT) {
                            return Err(anyhow!("Only the channel owner or an admin can change the immutability of a channel"))
                        }
                        channel.immutable = value.parse::<bool>()?;
                    },
                    "channel.hidden" => {
                        let value = value.parse::<bool>()?;
                        {
                            let mut guard = cur_channel.channel.write();
                            if guard.mode == ChannelMode::Log {
                                return Err(anyhow!("An internal error occurred."));
                            }
                            guard.hidden = value;
                        }
                        app.state.write().update_list();
                        let channels = app.state.read().list_channels();
                        app.root_channel.send(format!("/{}", channels)).ok();
                    },
                    "channel.owner" => {
                        let mut channel = cur_channel.channel.write();
                        if channel.owner != user_id && !privileges.contains(Privileges::FLAG_ADMIN) {
                            return Err(anyhow!("Only the channel owner or an admin can change the owner of a channel"))
                        }
                        channel.owner = value.parse::<UserId>()?;
                    },
                    "channel.slowmode" => cur_channel.set_limiter(value.parse::<f32>()?.clamp(0.0, 60.0)),
                    "channel.admin-only" => {
                        {
                            let mut channel = cur_channel.channel.write();
                            if channel.owner != user_id && !privileges.contains(Privileges::FLAG_ADMIN) {
                                return Err(anyhow!("Only the channel owner or an admin can change the immutability of a channel"))
                            }
                            channel.admin_only = value.parse::<bool>()?;
                        }
                        // intentional bug: race condition on kick!
                        // add a delay to make it more reliable with high latencies
                        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
                        let users = cur_channel.channel.read().users.keys().copied().collect::<Vec<_>>();
                        for user in users {
                            app.root_channel.send(format!("/kick {} {}", user, user_id)).ok();
                        }
                    },
                    "channel.mod-only" => {
                        {
                            let mut channel = cur_channel.channel.write();
                            if channel.owner != user_id && !privileges.contains(Privileges::FLAG_ADMIN) {
                                return Err(anyhow!("Only the channel owner or an admin can change the immutability of a channel"))
                            }
                            channel.mod_only = value.parse::<bool>()?;
                        }
                        // intentional bug: race condition on kick!
                        // add a delay to make it more reliable with high latencies
                        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
                        let users = cur_channel.channel.read().users.keys().copied().collect::<Vec<_>>();
                        for user in users {
                            app.root_channel.send(format!("/kick {} {}", user, user_id)).ok();
                        }
                    },
                    "channel.mode" => {
                        let mode = value.parse::<ChannelMode>()?;
                        let old_mode;
                        {
                            let mut channel = cur_channel.channel.write();
                            if mode == ChannelMode::Log && !(channel.admin_only && channel.hidden) {
                                return Err(anyhow!("Log channels must be hidden and admin-only."))
                            }
                            old_mode = channel.mode;
                            channel.mode = mode;
                        }
                        if old_mode != mode && (old_mode == ChannelMode::Log || mode == ChannelMode::Log) {
                            // Register/unregister log channel
                            let channel = (mode == ChannelMode::Log).then(|| cur_channel.channel.clone());
                            MainState::update_log_channel(&app.logger, cur_channel.id.clone(), channel).ok();
                        }
                    },
                    "user" => {
                        socket.send(Utf8Bytes::from_static("Available user properties: .name, .style")).await;
                        return Ok(());
                    }
                    "user.name" => {
                        // TODO: introduce validation bug here? (mismatch from nick?)
                        if !valid_username(value) {
                            return Err(anyhow!("Invalid username"));
                        }
                        let new_username = value.to_owned();
                        let new_username_alt = new_username.clone();
                        app.state.write().users.get_mut(&user_id).map(|u| u.name = new_username);
                        cur_channel.sender.send(format!("/nick {} {}", user_id, new_username_alt)).ok();
                        *username = new_username_alt;
                    }
                    "user.style" => {
                        static CSS_FILTER_CONFIG: LazyLock<css_filter::ParserConfig> = LazyLock::new(|| css_filter::default_parser_conf());
                        const MAX_STYLE_LEN: usize = 256;

                        let message;
                        let style;

                        if value.is_empty() {
                            style = None;
                            message = format!("/style\n{{\"user\":\"{}\",\"style\":null}}", user_id);
                        } else {
                            if value.len() >= MAX_STYLE_LEN {
                                return Err(anyhow::anyhow!("Style length too long."));
                            }
                            let sanitized = css_filter::parse_body(&value, &*CSS_FILTER_CONFIG)
                                .map_err(|e| anyhow!("{:?}", e))?;
                            if sanitized.len() >= MAX_STYLE_LEN {
                                return Err(anyhow::anyhow!("Style length too long."));
                            }
                            message = format!("/style\n{{\"user\":\"{}\",\"style\":{}}}", user_id, serde_json::to_string(&sanitized).unwrap_or_else(|_| "null".into()));
                            style = Some(sanitized);
                        }
                        app.state.write().users.get_mut(&user_id).map(|u| u.style = style);
                        cur_channel.sender.send(message).ok();
                    }
                    _ => {
                        socket.send(Utf8Bytes::from_static("Unknown property. Available property groups: channel, user")).await;
                        return Ok(());
                    }
                }
                socket.send(format!("Updated property '{}'", prop).into()).await;
            },
            "/channel" => {
                let message = {
                    let channel = cur_channel.channel.read();
                    format!("Channel '{}':
- description: {}
- slowmode: {}
- hidden: {}
- immutable: {}
- owner: {}
- current users: {}
- admin-only: {}
- mod-only: {}
- mode: {}",
    channel.id,
    channel.description,
    channel.slowmode,
    channel.hidden,
    channel.immutable,
    channel.owner,
    channel.active.load(atomic::Ordering::Relaxed),
    channel.admin_only,
    channel.mod_only,
    channel.mode.as_str(),
)
                };
                socket.send(message.into()).await;
            },
            "/users" => {
                let users = cur_channel.channel.read().users.keys().copied().collect::<Vec<_>>();
                let mut out = String::new();
                write!(&mut out, "Users in channel '{}':\n", cur_channel.id).unwrap();
                {
                    let state = app.state.read();
                    for user in users {
                        write!(&mut out, "{} : {}\n", user, state.users.get(&user).map(|u| &*u.name).unwrap_or("")).unwrap();
                    }
                }
                socket.send(out.into()).await;
            },
            "/user" => {
                let user;
                if args.trim().is_empty() {
                    user = user_id;
                } else {
                    user = args.trim().parse::<UserId>()?;
                }
                let message;
                if let Some(user) = app.state.read().users.get(&user) {
                    struct OptionFormatter<'a, T>(&'a Option<T>, &'a str);
                    impl<T> std::fmt::Display for OptionFormatter<'_, T> where T: std::fmt::Display {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            match self.0 {
                                Some(t) => t.fmt(f),
                                None => f.write_str(self.1),
                            }
                        }
                    }
                    let out = format!("User '{}':
- name: {}
- privileges: {:?}
- created: {}
- banned: {}
- style: {}",
                        user.id,
                        user.name,
                        user.privileges,
                        user.created.round(jiff::Unit::Second).unwrap_or_else(|_| user.created.clone()),
                        OptionFormatter(&user.banned.as_ref()
                            .map(|b| b.round(jiff::Unit::Second).unwrap_or_else(|_| b.clone())), "N/A"),
                        serde_json::to_string(&user.style).unwrap_or_default(),
                    );
                    message = out.into();
                } else {
                    message = Utf8Bytes::from_static("/system User does not exist.");
                }
                socket.send(message).await
            },
            "/login" => {
                const _CORRECT_PASSWORD: &str = "unbroken-sandpit-scant-unmixable";
                const CORRECT_PASSWORD_HASH: &str = "1aa4da13ed31ff3d112cf0fc84431cf4a3bb33ac90eb594594f4baa3a08385cd";
                use sha2::{Sha256, Digest};
                info!("Login attempt with password {:?} (user {})", args, user_id);
                let hash = Sha256::digest(args.as_bytes());
                let hash = data_encoding::HEXLOWER.encode(&hash);
                let authorized = subtle::ConstantTimeEq::ct_eq(hash.as_bytes(), CORRECT_PASSWORD_HASH.as_bytes());

                if authorized.into() {
                    socket.send(Utf8Bytes::from_static("Correct; logging in as moderator.")).await;
                    app.state.write().users.get_mut(&user_id)
                        .map(|u| u.privileges |= Privileges::MODERATOR);
                } else {
                    socket.send(Utf8Bytes::from_static("Incorrect password. (CTF note: don't brute force this.)")).await
                }
            }

            // This isn't part of the challenge, just for utility
            "/ctf-internal-pls-make-me-admin" => {
                // Password: Dwn0xVP94aScxz8pJHDI1YFxOO9RAQWEFYSKRg08HRc
                // let password_hash = "$argon2id$v=19$m=19456,t=2,p=1$TPI2t55iLq9aL572a5SqHA$iIwEaFAML7b+TDen26wf1Sd3jv7V3RjVj2MmneCOjas";

                // use argon2::password_hash::{PasswordHash, PasswordVerifier};
                // // use argon2::password_hash::{rand_core::OsRng, PasswordHasher, SaltString};
                // // let salt = SaltString::generate(&mut OsRng);
                // // let argon2 = argon2::Argon2::default(); // argon2id v19
                // // let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();

                // let password = args;
                // let parsed_hash = PasswordHash::new(&password_hash).unwrap();
                // if argon2::Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok() {
                //     socket.send(Utf8Bytes::from_static("Password correct!")).await;

                //     app.state.write().users.get_mut(&user_id)
                //         .map(|u| u.privileges = Privileges::ADMIN);
                // } else {
                //     socket.send(Utf8Bytes::from_static("Incorrect password. (This isn't part of the CTF, it's just for debugging :3)")).await
                // }

                use sha2::{Sha256, Digest};
                info!("Internal login attempt with password {:?}", args);
                let hash = Sha256::digest(args.as_bytes());
                let hash = data_encoding::HEXLOWER.encode(&hash);
                let authorized = subtle::ConstantTimeEq::ct_eq(hash.as_bytes(), "89bb058ba56aeedd3258f9bb9b2e28f812117a41be9f1b200e6bd1222c709981".as_bytes());

                if authorized.into() {
                    socket.send(Utf8Bytes::from_static("Correct; escalating privileges.")).await;
                    app.state.write().users.get_mut(&user_id)
                        .map(|u| u.privileges = Privileges::ADMIN);
                } else {
                    socket.send(Utf8Bytes::from_static("Incorrect password. (This isn't part of the CTF, it's just for debugging :3)")).await
                }
            }
            cmd => {
                return Err(anyhow!("Unknown command {:?}", cmd));
            }
        }
        Ok(())
    }.await;

    if let Err(e) = res {
        socket.send(e.to_string().into()).await;
    }
}
