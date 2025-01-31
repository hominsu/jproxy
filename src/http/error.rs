#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Http(#[from] http::Error),

    #[error(transparent)]
    Hyper(#[from] hyper::Error),

    #[error(transparent)]
    HyperLegacy(#[from] hyper_util::client::legacy::Error),

    #[error(transparent)]
    Timeout(#[from] tokio::time::error::Elapsed),

    #[error(transparent)]
    Connect(#[from] crate::connect::error::Error),
}
