#[derive(thiserror::Error, Debug)]
#[error("dns error: {0}")]
pub struct DnsError(#[from] pub std::io::Error);

#[derive(thiserror::Error, Debug)]
#[error("invalid uri: {0}")]
pub struct InvalidUriError(#[from] pub std::io::Error);

#[derive(thiserror::Error, Debug)]
#[error("tcp error: {0}")]
pub struct TcpError(#[from] pub std::io::Error);

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    HyperLegacy(#[from] hyper_util::client::legacy::Error),

    #[error(transparent)]
    Dns(#[from] DnsError),

    #[error(transparent)]
    InvalidUri(#[from] InvalidUriError),

    #[error(transparent)]
    Tcp(#[from] TcpError),
}
