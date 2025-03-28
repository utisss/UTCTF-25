import htm, { elem } from "./htm.js";
window.html = htm.bind(elem);
let html = window.html;

import { connect, AsyncWS } from "./async-ws.js";

(() => {
    let socket = null;
    window.joinChannel = (channel) => {
        socket.send(`/join ${channel}`)
    };

    let channelList = [];
    function createChannelList(list) {
        let elem = document.querySelector("#channel-list");
        elem.replaceChildren();
        for (let channel of list) {
            elem.appendChild(html`
                <li class="channel">
                    <a href="#" onclick="joinChannel('${channel}')">${channel}</a>
                </li>
            `);
        }
        channelList = list;
    }

    let styles = document.createElement("style");
    document.head.appendChild(styles);
    let stylesheet = styles.sheet;
    let userStyles = {};

    function updateStyles(users) {
        // console.log(users);
        for (let user of users) {
            try {
                if (user.style == null) continue;
                let existing = userStyles[user.user];
                let style = `.msg[data-user="${user.user}"]{${user.style}}`;
                if (existing != null) {
                    stylesheet.deleteRule(existing);
                    let idx = stylesheet.insertRule(style, existing)
                    console.assert(idx == existing);
                } else {
                    let idx = stylesheet.insertRule(style, stylesheet.cssRules.length);
                    userStyles[user.user] = idx;
                }
            } catch (e) {
                console.error(e);
            }
        }
    }

    let chat = document.querySelector("#chat");
    let chatbox = document.querySelector("#chatbox");

    chatbox.addEventListener("submit", e => {
        e.preventDefault();
        let data = new FormData(chatbox);
        let message = data.get("message");
        if (message.startsWith("/")) {
            socket.send(message);
        } else if (message != "") {
            socket.send("/msg " + message);
        }
        chatbox.reset();
    });

    function parseArgs(str, count) {
        let args = [];
        let base = 0;
        for (let i = 0; i < count; i++) {
            let arg_end = str.indexOf(" ", base);
            if (arg_end == -1 || i == count - 1) {
                args.push(str.slice(base));
                base = str.length;
            } else {
                args.push(str.slice(base, arg_end));
                base = Math.min(arg_end + 1, str.length);
            }
        }
        return args;
    }

    async function socketHandler() {
        console.log("Starting socket");
        let ws_url = new URL("./socket", window.location.href);
        ws_url.protocol = ws_url.protocol.replace("http", "ws");
        socket = await connect(ws_url);

        socket.send("/list");
        let data;
        try {
            while (data = await socket.recv()) {
                try {
                    console.log(data);
                    let [cmd, args] = parseArgs(data, 2);
                    if (cmd == "/msg") {
                        let [user, username, msg] = parseArgs(args, 3);
                        chat.appendChild(html`<div class="msg" data-user="${user}">
                            <span class="username">${username}</span>
                            <span class="ws"> </span>
                            <span class="content">${msg}</span>
                        </div>`);
                    } else if (cmd == "/announce") {
                        let [user, username, msg] = parseArgs(args, 3);
                        chat.appendChild(html`<div class="msg announcement" data-user="${user}">
                            <span class="username">${username}</span>
                            <span class="ws"> </span>
                            <span class="content">${msg}</span>
                        </div>`);
                    } else if (cmd == "/nick") {
                        let [user, username] = parseArgs(args, 2);
                        chat.appendChild(html`<div class="msg system" data-user="${user}">
                            <span class="content">User ${user} updated their name to "${username}"</span>
                        </div>`);
                    } else if (cmd == "/channels") {
                        let channels = args.trim() == "" ? [] : args.trim().split(" ");
                        createChannelList(channels);
                        chat.appendChild(html`<div class="msg system">
                            <span class="content">Available channels: ${channels.join(", ")}</span>
                        </div>`);
                    } else if (cmd == "//channels") {
                        let channels = args.trim() == "" ? [] : args.trim().split(" ");
                        createChannelList(channels);
                    } else if (cmd == "/style" || data.startsWith("/style\n")) {
                        args = data.slice(data.indexOf("\n"));
                        let users = args.trim() == "" ? [] : args.trim().split("\n");
                        users = users.map(e => JSON.parse(e));
                        updateStyles(users);
                    } else if (cmd == "/create") {
                        let [channel, user_id, username] = parseArgs(args, 3);
                        if (!channel.startsWith("_")) {
                            chat.appendChild(html`<div class="msg system">
                                <span class="content">Channel ${channel} created by ${username} (${user_id})</span>
                            </div>`);
                            channelList.push(channel);
                            channelList.sort();
                            createChannelList(channelList);
                        }
                    } else if (cmd == "/delete") {
                        let [channel, user_id, username] = parseArgs(args, 3);
                        if (!channel.startsWith("_")) {
                            chat.appendChild(html`<div class="msg system">
                                <span class="content">Channel ${channel} deleted by ${username} (${user_id})</span>
                            </div>`);
                        }
                        channelList = channelList.filter(c => c != channel);
                        channelList.sort();
                        createChannelList(channelList);
                    } else if (cmd == "/join") {
                        let [user_id, user] = parseArgs(args, 2);
                        chat.appendChild(html`<div class="msg join" data-user="${user_id}">
                            <span class="content">${user} joined the channel (${user_id})</span>
                        </div>`);
                    } else if (cmd == "/leave") {
                        let [user_id, user] = parseArgs(args, 2);
                        chat.appendChild(html`<div class="msg leave" data-user="${user_id}">
                            <span class="content">${user} left the channel (${user_id})</span>
                        </div>`);
                    } else if (cmd == "/kick") {
                        let [target, user] = parseArgs(args, 2);
                        chat.appendChild(html`<div class="msg system">
                            <span class="content">${target} was kicked by ${user}</span>
                        </div>`);
                    } else if (cmd == "/ban") {
                        let [target, user] = parseArgs(args, 2);
                        chat.appendChild(html`<div class="msg system">
                            <span class="content">${target} was banned by ${user}</span>
                        </div>`);
                    } else if (cmd == "/system") {
                        chat.appendChild(html`<div class="msg system">
                            <span class="content">${args}</span>
                        </div>`);
                    } else if (cmd == "/error") {
                        chat.appendChild(html`<div class="msg system sys-error">
                            <span class="content">${args}</span>
                        </div>`);
                    } else {
                        chat.appendChild(html`<div class="msg system sys-unknown">
                            <span class="content">${data}</span>
                        </div>`);
                    }
                } catch (e) {
                    console.error(e);
                }
            }
        } catch (e) {
            console.error("Error recieving from socket:", e);
        }
        try { socket.close() } catch (e) {}
        setTimeout(socketHandler, 5000 + Math.random() * 5000);
    }

    setTimeout(socketHandler, 0);
})();
