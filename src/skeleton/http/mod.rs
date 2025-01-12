use bytes::Bytes;
use http::{Method, StatusCode};
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::{
    body::Incoming, client::conn::http1::Builder, service::Service, upgrade::Upgraded, Request,
    Response,
};
use hyper_util::rt::TokioIo;
use std::{future::Future, io, pin::Pin};
use tokio::net::TcpStream;

mod error;
pub use error::Error;

#[derive(Debug, Clone)]
pub struct HttpProxy {}

impl Service<Request<Incoming>> for HttpProxy {
    type Response = Response<BoxBody<Bytes, hyper::Error>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let proxy = self.clone();

        Box::pin(async move {
            match *req.method() {
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
                _ => {
                    tracing::info!("{req:?}");

                    let host = req.uri().host().expect("uri has no host");
                    let port = req.uri().port_u16().unwrap_or(80);

                    let stream = TcpStream::connect((host, port)).await.unwrap();
                    let io = TokioIo::new(stream);

                    let (mut sender, conn) = Builder::new()
                        .preserve_header_case(true)
                        .title_case_headers(true)
                        .handshake(io)
                        .await?;
                    tokio::task::spawn(async move {
                        if let Err(err) = conn.await {
                            tracing::warn!("Connection failed: {:?}", err)
                        }
                    });

                    let resp = sender.send_request(req).await?;
                    Ok(resp.map(|b| b.boxed()))
                }
            }
        })
    }
}

impl HttpProxy {
    async fn tunnel(&self, upgraded: Upgraded, addr: String) -> io::Result<()> {
        let mut server = TcpStream::connect(addr).await?;
        let mut upgraded = TokioIo::new(upgraded);

        let (from_client, from_server) =
            tokio::io::copy_bidirectional(&mut upgraded, &mut server).await?;
        tracing::debug!(
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
