use futures_util::TryFutureExt;
use http::Uri;
use hyper_util::{client::legacy::connect::HttpConnector as HyperHttpConnector, rt::TokioIo};
use std::{
    future::Future,
    io,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::net::TcpStream;
use tower_service::Service;

mod error;

#[derive(Clone)]
pub struct HttpConnector {
    inner: HyperHttpConnector,
}

impl HttpConnector {
    pub fn new() -> Self {
        Self {
            inner: HyperHttpConnector::new(),
        }
    }
}

impl Service<Uri> for HttpConnector {
    type Response = TokioIo<TcpStream>;
    type Error = error::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner
            .poll_ready(cx)
            .map_err(|e| error::Error::Connect(io::Error::new(io::ErrorKind::Other, e.to_string())))
    }

    fn call(&mut self, uri: Uri) -> Self::Future {
        Box::pin(self.inner.call(uri).map_err(|e| {
            error::Error::Connect(io::Error::new(io::ErrorKind::Other, e.to_string()))
        }))
    }
}
