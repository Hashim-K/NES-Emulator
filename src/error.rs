use thiserror::Error;

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

