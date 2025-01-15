#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    HyperLegacy(#[from] hyper_util::client::legacy::Error),

    #[error(transparent)]
    Connect(#[from] std::io::Error),
}
