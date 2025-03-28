#[macro_use]
extern crate tracing;

use anyhow::{anyhow, Context};
use chat as web;

use runtime::utils::enclose;

struct Args {
    bind_addr: std::net::SocketAddr,
}

fn parse_args(args: impl Iterator<Item = String>) -> Result<Option<Args>, anyhow::Error> {
    let mut bind_addr = None;

    let res = runtime::args::parse_args(
        args,
        |flag, inline, _args, _arg0| match (flag, inline) {
            (c, _) => Err(anyhow!("Unknown flag '-{}'", c)),
        },
        |index, arg| {
            match index {
                0 => {
                    let addr = arg
                        .parse::<std::net::SocketAddr>()
                        .context("Failed to parse bind address")?;
                    bind_addr = Some(addr);
                }
                _ => return Err(anyhow!("Only one positional argument is allowed")),
            }
            Ok(Some(()))
        },
    )?;
    if res.is_none() {
        return Ok(None);
    }

    Ok(Some(Args {
        bind_addr: bind_addr.context("Missing bind address argument")?,
    }))
}

fn print_help_message(arg0: &str) {
    println!(
        "Usage: {arg0} [flags] addr

Args:
    addr                    the bind address for the webserver
"
    );
}

fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();

    runtime::log::setup_logger(module_path!())?;

    let args = match parse_args(std::env::args()) {
        Ok(Some(args)) => args,
        Ok(None) => return Ok(()),
        Err(e) => {
            eprintln!("{}", e);
            print_help_message(std::env::args().next().as_deref().unwrap_or("unknown"));
            std::process::exit(1);
        }
    };

    let run_handle = runtime::RunHandle::new();

    let tasks = vec![(
        "webserver",
        runtime::handler(enclose!([] move |cancel| async move {
            let result = web::start_webserver(args.bind_addr, cancel).await;
            if let Err(e) = &result {
                error!("Webserver encountered error: {:?}", e);
            }
        })),
    )];

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    rt.block_on(runtime::run(run_handle, tasks, Box::new(|| {})))?;

    Ok(())
}
