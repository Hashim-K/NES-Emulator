use thiserror::Error;

#[derive(Debug, Error)]
pub enum RomError {
    #[error("Unknown Mapper Error: Mapper {0} is not implemented")]
    UnknownMapper(u8),
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
    #[error("Rom Error occurred: {0}")]
    RomError(#[from] RomError),
    #[error("Unknown Error: {0}")]
    Unknown(String),
}

#[derive(Debug, Error)]
pub enum MainError {
    #[error("Get Cpu Error occurred: {0}")]
    MyGetCpuError(#[from] MyGetCpuError),
}

