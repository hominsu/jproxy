mod error;
pub use error::Error;

use super::{config::Config, connect::HttpConnector};
use bytes::Bytes;
use http::{Method, StatusCode};
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::{body::Incoming, upgrade::Upgraded, Request, Response};
use hyper_util::{
    client::legacy::Client,
    rt::{TokioExecutor, TokioIo},
};
use std::{
    future::Future,
    io,
    pin::Pin,
    sync::{Arc, RwLock},
    task::{Context, Poll},
};
use tokio::net::TcpStream;
use tower_service::Service;

#[derive(Debug, Clone)]
pub struct HttpProxy {
    #[allow(dead_code)]
    config: Arc<RwLock<Config>>,
}

impl Service<Request<Incoming>> for HttpProxy {
    type Response = Response<BoxBody<Bytes, hyper::Error>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Incoming>) -> Self::Future {
        let proxy = self.clone();

        Box::pin(async move {
            tracing::info!("{req:?}");
            match *req.method() {
                // Handles HTTPS connections by establishing a tunnel via the CONNECT method
                Method::CONNECT => {
                    if let Some(addr) = host_addr(req.uri()) {
                        tokio::task::spawn(async move {
                            match hyper::upgrade::on(req).await {
                                Ok(upgraded) => {
                                    if let Err(e) = proxy.tunnel(upgraded, addr).await {
                                        tracing::warn!("server io error: {}", e);
                                    }
                                }
                                Err(e) => tracing::warn!("upgrade error: {}", e),
                            }
                        });

                        Ok(Response::new(empty()))
                    } else {
                        tracing::warn!("CONNECT host is not socket addr: {:?}", req.uri());
                        let mut resp = Response::new(full("CONNECT must be to a socket address"));
                        *resp.status_mut() = StatusCode::BAD_REQUEST;

                        Ok(resp)
                    }
                }
                // Handles regular HTTP connections by forwarding the request to the destination
                _ => {
                    let connector = HttpConnector::new();
                    let resp = Client::builder(TokioExecutor::new())
                        .http1_preserve_header_case(true)
                        .http1_title_case_headers(true)
                        .build(connector)
                        .request(req)
                        .await?;

                    Ok(resp.map(|b| b.boxed()))
                }
            }
        })
    }
}

impl HttpProxy {
    pub fn new(config: Arc<RwLock<Config>>) -> Self {
        Self { config }
    }

    async fn tunnel(&self, upgraded: Upgraded, addr: String) -> io::Result<()> {
        let mut server = TcpStream::connect(addr).await?;
        let mut upgraded = TokioIo::new(upgraded);

        let (from_client, from_server) =
            tokio::io::copy_bidirectional(&mut upgraded, &mut server).await?;
        tracing::trace!(
            "client wrote {} bytes and received {} bytes",
            from_client,
            from_server
        );

        Ok(())
    }
}

fn host_addr(uri: &http::Uri) -> Option<String> {
    uri.authority().map(|auth| auth.to_string())
}

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
