use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum RomError {
    #[error("Unknown Mapper Error: Mapper {0} is not implemented")]
    UnknownMapper(u8),
    #[error("Unknown Address Error: Rom address not in the right range")]
    UnknownAddress,
    #[error("Header signature does not match specification")]
    IncorrectSignature,
    #[error("Unknown Error: {0}")]
    Unknown(String),
    #[error("Given amount of data does not match header")]
    IncorrectDataSize,
}

#[derive(Debug, Error)]
pub enum MemoryError {
    #[error("Rom Error occurred: {0}")]
    RomError(#[from] RomError),
    #[error("Unknown Address Error: Memory address not in the right range")]
    UnknownAddress,
}

#[derive(Debug, Error)]
pub enum MyTickError {
}

#[derive(Debug, Error)]
pub enum MyGetCpuError {
    #[error("Rom Error occurred: {0}")]
    RomError(#[from] RomError),
}

#[derive(Debug, Error)]
pub enum MainError {
    #[error("Get Cpu Error occurred: {0}")]
    MyGetCpuError(#[from] MyGetCpuError),

    #[error("Memory Error occurred: {0}")]
    MemoryError(#[from] MemoryError),
}

