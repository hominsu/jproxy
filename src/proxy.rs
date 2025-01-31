use std::future::IntoFuture;
use std::sync::Arc;

use tokio::sync::watch;
use tokio::{signal, sync::watch::Sender};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::{manager, Config};
use crate::http::HttpProxy;
use crate::serve::serve;
use crate::Bootstrap;

pub async fn shutdown_signal(tx: Arc<Sender<()>>) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
        _ = tx.closed() => {},
    }
}

pub fn run(args: Bootstrap) -> crate::Result<()> {
    let manager = manager(args.conf.as_str());
    let config = manager.config();
    let Config {
        debug,
        bind,
        concurrent,
        ..
    } = config.read().unwrap().clone();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}={}",
                    env!("CARGO_CRATE_NAME"),
                    if debug { "trace" } else { "info" }
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .max_blocking_threads(concurrent)
        .build()?;

    runtime.block_on(async move {
        let http_proxy = HttpProxy::new(config);
        let listener = tokio::net::TcpListener::bind(bind).await.unwrap();

        tracing::info!("listening on {}", listener.local_addr().unwrap());

        let (tx, rx) = watch::channel(());
        let tx = Arc::new(tx);

        tokio::pin! {
            let serve_fut = serve(listener, http_proxy)
                .with_graceful_shutdown(shutdown_signal(Arc::clone(&tx)))
                .into_future();

            let manager_fut = manager
                .with_watcher(shutdown_signal(Arc::clone(&tx)))
                .into_future();
        }

        tokio::select! {
            _ = &mut serve_fut => {
                drop(rx);
                let _ = &mut manager_fut.await;
            },
            _ = &mut manager_fut => {
                drop(rx);
                let _ = &mut serve_fut.await;
            },
        }
    });

    Ok(())
}
