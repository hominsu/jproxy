use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::{body::Incoming, service::Service, upgrade::Upgraded, Request, Response};
use hyper_util::rt::TokioIo;
use std::{future::Future, io, pin::Pin, sync::Arc};
use tokio::net::TcpStream;

#[derive(Debug, Clone)]
struct HttpProxy(Arc<()>);

impl Service<Request<Incoming>> for HttpProxy {
    type Response = Response<BoxBody<Bytes, Self::Error>>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, _req: Request<Incoming>) -> Self::Future {
        todo!()
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
