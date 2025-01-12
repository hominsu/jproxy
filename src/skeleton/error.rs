#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    HttpError(#[from] super::http::Error),
}
