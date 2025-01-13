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

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .max_blocking_threads(args.concurrent)
        .build()?;

    runtime.block_on(async move {
        let manager = manager(args.conf.as_str()).with_watcher(shutdown_signal());
        let config = manager.config();
        let manager_fut = manager.into_future();

        let http_proxy = crate::skeleton::http::HttpProxy {};
        let addr = config.read().unwrap().bind.clone();
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

        tracing::info!("listening on {}", listener.local_addr().unwrap());

        let serve_fut = crate::skeleton::serve::serve(listener, http_proxy)
            .with_graceful_shutdown(shutdown_signal())
            .into_future();

        let _ = tokio::join!(serve_fut, manager_fut);
    });

    Ok(())
}
