#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    HttpError(#[from] http::Error),

    #[error(transparent)]
    HyperError(#[from] hyper::Error),

    #[error(transparent)]
    Timeout(#[from] tokio::time::error::Elapsed),
}
