use std::future::{Future, IntoFuture};
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use futures_util::{pin_mut, FutureExt};
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder;
use hyper_util::service::TowerToHyperService;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::watch;

use crate::http::HttpProxy;

pub fn serve(tcp_listener: TcpListener, http_proxy: HttpProxy) -> Serve {
    Serve {
        tcp_listener,
        http_proxy,
    }
}

pub struct Serve {
    tcp_listener: TcpListener,
    http_proxy: HttpProxy,
}

impl Serve {
    pub fn with_graceful_shutdown<F>(self, signal: F) -> WithGracefulShutdown<F>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        WithGracefulShutdown {
            tcp_listener: self.tcp_listener,
            http_proxy: self.http_proxy,
            signal,
        }
    }
}

impl IntoFuture for Serve {
    type Output = io::Result<()>;
    type IntoFuture = private::ServeFuture;

    fn into_future(self) -> Self::IntoFuture {
        self.with_graceful_shutdown(std::future::pending())
            .into_future()
    }
}

pub struct WithGracefulShutdown<F> {
    tcp_listener: TcpListener,
    http_proxy: HttpProxy,
    signal: F,
}

impl<F> IntoFuture for WithGracefulShutdown<F>
where
    F: Future<Output = ()> + Send + 'static,
{
    type Output = io::Result<()>;
    type IntoFuture = private::ServeFuture;

    fn into_future(self) -> Self::IntoFuture {
        private::ServeFuture(Box::pin(async move {
            self.run().await;
            Ok(())
        }))
    }
}

impl<F> WithGracefulShutdown<F>
where
    F: Future<Output = ()> + Send + 'static,
{
    async fn run(self) {
        let Self {
            tcp_listener,
            http_proxy,
            signal,
        } = self;

        let (signal_tx, signal_rx) = watch::channel(());
        let signal_tx = Arc::new(signal_tx);
        tokio::spawn(async move {
            signal.await;
            tracing::trace!("received graceful shutdown signal. Telling tasks to shutdown");
            drop(signal_rx);
        });

        let (close_tx, close_rx) = watch::channel(());

        loop {
            let (io, remote_addr) = tokio::select! {
                conn = tcp_accept(&tcp_listener) => {
                    match conn {
                        Some(conn) => conn,
                        None => continue,
                    }
                }
                _ = signal_tx.closed() => {
                    tracing::trace!("signal received, not accepting new connections");
                    break;
                }
            };

            let mut version_buffer = [0u8; 1];
            match io.peek(&mut version_buffer).await {
                Ok(n) => {
                    if n == 0 {
                        tracing::warn!("connection closed before reading version");
                        continue;
                    }
                }
                Err(err) => {
                    tracing::warn!("failed to read version: {err:#}");
                    continue;
                }
            }

            let io = TokioIo::new(io);

            tracing::trace!("connection {remote_addr:?} accepted");

            match version_buffer[0] {
                b'G' | b'g' |   // GET
                b'H' | b'h' |   // HEAD
                b'P' | b'p' |   // POST
                b'D' | b'd' |   // DELETE
                b'C' | b'c' |   // CONNECT
                b'O' | b'o' |   // OPTIONS
                b'T' | b't'     // TRACE
                => {
                    let hyper_service = TowerToHyperService::new(http_proxy.clone());

                    let signal_tx = Arc::clone(&signal_tx);
                    let close_rx = close_rx.clone();

                    tokio::spawn(async move {
                        #[allow(unused_mut)]
                        let mut builder = Builder::new(TokioExecutor::new());
                        let conn = builder.serve_connection_with_upgrades(io, hyper_service);
                        pin_mut!(conn);

                        let signal_closed = signal_tx.closed().fuse();
                        pin_mut!(signal_closed);

                        loop {
                            tokio::select! {
                                result = conn.as_mut() => {
                                    if let Err(_err) = result {
                                        tracing::trace!("Failed to serve connection: {_err:#}");
                                    }
                                    break;
                                }
                                _ = &mut signal_closed => {
                                    tracing::trace!("signal received in task, starting graceful shutdown");
                                    conn.as_mut().graceful_shutdown();
                                }
                            }
                        }

                        drop(close_rx);
                    });
                }
                version => tracing::warn!("unsupported version: {:x}", version),
            }
        }

        drop(close_rx);
        drop(tcp_listener);

        tracing::trace!(
            "waiting for {} task(s) to finish",
            close_tx.receiver_count()
        );
        close_tx.closed().await;
    }
}

fn is_connection_error(e: &io::Error) -> bool {
    matches!(
        e.kind(),
        io::ErrorKind::ConnectionRefused
            | io::ErrorKind::ConnectionAborted
            | io::ErrorKind::ConnectionReset
    )
}

async fn tcp_accept(listener: &TcpListener) -> Option<(TcpStream, SocketAddr)> {
    match listener.accept().await {
        Ok(conn) => Some(conn),
        Err(err) => {
            if is_connection_error(&err) {
                return None;
            }

            tracing::error!("accept error: {:#?}", err);
            tokio::time::sleep(Duration::from_secs(1)).await;
            None
        }
    }
}

mod private {
    use std::{
        future::Future,
        io,
        pin::Pin,
        task::{Context, Poll},
    };

    pub struct ServeFuture(pub(super) futures_util::future::BoxFuture<'static, io::Result<()>>);

    impl Future for ServeFuture {
        type Output = io::Result<()>;

        #[inline]
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            self.0.as_mut().poll(cx)
        }
    }
}
