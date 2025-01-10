mod skeleton;
use argh::FromArgs;
use std::future::IntoFuture;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(FromArgs)]
#[argh(description = "Just a proxy")]
struct Args {
    #[argh(
        option,
        short = 'c',
        default = "String::from(\"configs\")",
        description = "config path, eg: --conf ./configs"
    )]
    conf: String,

    #[argh(switch, short = 'v', description = "print version and quit")]
    version: bool,
}

#[tokio::main]
async fn main() {
    let args: Args = argh::from_env();

    if args.version {
        println!("http-proxy-ipv6-balancer v{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=trace,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let manager = skeleton::manager(args.conf.as_str()).with_watcher(skeleton::shutdown_signal());
    let _config = manager.config();
    let manager_fut = manager.into_future();

    let _ = tokio::join!(manager_fut);
}
