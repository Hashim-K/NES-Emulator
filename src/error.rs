use thiserror::Error;

#[derive(Debug, Error)]
pub enum RomError {
    #[error("Unknown Address Error: Rom address not in the right range")]
    UnknownAddress,
    #[error("Header signature does not match specification")]
    IncorrectSignature,
    #[error("Unknown Error: {0}")]
    Unknown(String),
}

#[derive(Debug, Error)]
pub enum MyTickError {
    /// TODO: change this
    #[error("Unknown Error: {0}")]
    Unknown(String),
}

#[derive(Debug, Error)]
pub enum MyGetCpuError {
    /// TODO: change this
    #[error("Unknown Error: {0}")]
    Unknown(String),
}

