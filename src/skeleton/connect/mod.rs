use http::Uri;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::TokioIo;
use std::{
    future::Future,
    io,
    net::SocketAddr,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::net::TcpStream;
use tower_service::Service;

#[derive(Clone)]
pub struct Connector(HttpConnector);

impl Connector {
    pub fn new() -> Self {
        Connector(HttpConnector::new())
    }
}

impl Service<Uri> for Connector {
    type Response = TokioIo<TcpStream>;
    type Error = io::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _dst: Uri) -> Self::Future {
        println!("{_dst:?}");
        Box::pin(async {
            let io = TcpStream::connect(SocketAddr::from(([127, 0, 0, 1], 1337))).await?;
            Ok(TokioIo::new(io))
        })
    }
}
