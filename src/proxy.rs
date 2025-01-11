use crate::skeleton::{manager, shutdown_signal};
use crate::Bootstrap;
use std::future::IntoFuture;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn run(args: Bootstrap) -> crate::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}={}",
                    env!("CARGO_CRATE_NAME"),
                    if args.debug { "debug" } else { "info" }
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let manager = manager(args.conf.as_str()).with_watcher(shutdown_signal());
    let _config = manager.config();

    let manager_fut = manager.into_future();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .max_blocking_threads(args.concurrent)
        .build()?
        .block_on(async {
            let _ = tokio::join!(manager_fut);
        });

    Ok(())
}
