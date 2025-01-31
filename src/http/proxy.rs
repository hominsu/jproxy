use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, RwLock};
use std::task::{Context, Poll};

use bytes::Bytes;
use http::{Method, StatusCode, Uri};
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::{body::Incoming, upgrade::Upgraded, Request, Response};
use hyper_util::client::legacy::Client;
use hyper_util::rt::{TokioExecutor, TokioIo};
use tokio::net::TcpStream;
use tower_service::Service;

use super::error::Error;
use crate::{config::Config, connect::tcp::TcpConnector};

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
                Method::CONNECT => proxy.connect(req).await,
                _ => proxy.http(req).await,
            }
        })
    }
}

impl HttpProxy {
    pub fn new(config: Arc<RwLock<Config>>) -> Self {
        Self { config }
    }

    async fn http(
        self,
        req: Request<Incoming>,
    ) -> Result<Response<BoxBody<Bytes, hyper::Error>>, Error> {
        // Handles regular HTTP connections by forwarding the request to the destination

        let Config {
            connect_timeout, ..
        } = self.config.read().unwrap().clone();

        let mut connector = TcpConnector::new();
        connector.set_connect_timeout(connect_timeout);

        let resp = Client::builder(TokioExecutor::new())
            .http1_preserve_header_case(true)
            .http1_title_case_headers(true)
            .build(connector)
            .request(req)
            .await?;

        Ok(resp.map(|b| b.boxed()))
    }

    async fn connect(
        self,
        req: Request<Incoming>,
    ) -> Result<Response<BoxBody<Bytes, hyper::Error>>, Error> {
        let uri = req.uri().clone();

        if host_addr(&uri).is_none() {
            tracing::warn!("CONNECT host is not socket addr: {}", uri);
            let mut resp = Response::new(full("CONNECT must be to a socket address"));
            *resp.status_mut() = StatusCode::BAD_REQUEST;

            return Ok(resp);
        }

        tokio::task::spawn(async move {
            match hyper::upgrade::on(req).await {
                Ok(upgraded) => {
                    if let Err(e) = self.establish_tunnel(upgraded, uri).await {
                        tracing::warn!("tunnel error: {}", e);
                    }
                }
                Err(e) => tracing::warn!("upgrade error: {}", e),
            }
        });

        // Immediately return a 200 that the tunnel is established
        Ok(Response::new(empty()))
    }

    async fn establish_tunnel(&self, upgraded: Upgraded, uri: Uri) -> Result<(), Error> {
        let Config {
            connect_timeout, ..
        } = self.config.read().unwrap().clone();

        let mut connector = TcpConnector::new();
        connector.set_connect_timeout(connect_timeout);

        futures_util::future::poll_fn(|cx| connector.poll_ready(cx)).await?;

        let server = connector.call(uri).await?.into_inner();

        self.tunnel(TokioIo::new(upgraded), server).await
    }

    async fn tunnel(
        &self,
        mut upgraded: TokioIo<Upgraded>,
        mut server: TcpStream,
    ) -> Result<(), Error> {
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

fn host_addr(uri: &Uri) -> Option<String> {
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
