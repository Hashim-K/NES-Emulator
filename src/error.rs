use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum RomError {
    #[error("Unknown Mapper Error: Mapper {0} is not implemented. Details: {1}")]
    UnknownMapper(u8, String),
    #[error("Unknown Address Error: Rom address not in the right range. Details: {0}")]
    UnknownAddress(String),
    #[error("Header signature does not match specification. Details: {0}")]
    IncorrectSignature(String),
    #[error("Given amount of data does not match header. Details: {0}")]
    IncorrectDataSize(String),
}

#[derive(Debug, Error)]
pub enum MemoryError {
    #[error("Rom Error occurred: {0}")]
    RomError(#[from] RomError),
    #[error("Unknown Address Error: Memory address not in the right range. Details: {0}")]
    UnknownAddress(String),
    #[error("Error in shift register data. Details: {0}")]
    ShiftAddressError(String),
    #[error("Error writing to address for MMC1 mapper: {0}. Details: {1}")]
    MapperAddressError(u16, String),
}

#[derive(Debug, Error)]
pub enum MyTickError {
    #[error("MainError occurred in one of the functions during a CPU tick. Details: {1}")]
    MainError(#[source] Box<MainError>, String),

    #[error("Memory error occurred in the tick function. Details: {1}")]
    MemoryError(#[source] Box<MemoryError>, String),
}

#[derive(Debug, Error)]
pub enum MyGetCpuError {
    #[error("Rom Error occurred: {0}")]
    RomError(#[from] RomError),
}

#[derive(Debug, Error)]
pub enum MainError {
    #[error("Get Cpu Error occurred. Details: {1}")]
    MyGetCpu(#[source] Box<MyGetCpuError>, String),

    #[error("Memory Error occurred. Details: {1}")]
    Memory(#[source] Box<MemoryError>, String),

    #[error("Opcode Error occurred. Details: {0}")]
    Opcode(String),
}

// Implement `From` conversions, passing along the string context from the source errors

impl From<MemoryError> for MainError {
    fn from(error: MemoryError) -> Self {
        let context = format!("{}", error); // Capture the context string from the MemoryError
        MainError::Memory(Box::new(error), context)
    }
}

impl From<MyGetCpuError> for MainError {
    fn from(error: MyGetCpuError) -> Self {
        let context = format!("{}", error); // Capture the context string from the MyGetCpuError
        MainError::MyGetCpu(Box::new(error), context)
    }
}

impl From<RomError> for MainError {
    fn from(error: RomError) -> Self {
        let context = format!("{}", error); // Capture the context string from the RomError
        MainError::MyGetCpu(Box::new(MyGetCpuError::RomError(error)), context)
    }
}

impl From<MyTickError> for MainError {
    fn from(error: MyTickError) -> Self {
        let context = format!("{}", error); // Capture the context string from the MyTickError
        MainError::Memory(
            Box::new(MemoryError::RomError(RomError::UnknownAddress(
                context.clone(),
            ))),
            context,
        )
    }
}

// New implementations for `MyTickError` conversions, passing along the string context

impl From<MemoryError> for MyTickError {
    fn from(error: MemoryError) -> Self {
        let context = format!("{}", error); // Capture the context string from the MemoryError
        MyTickError::MemoryError(Box::new(error), context)
    }
}

impl From<MainError> for MyTickError {
    fn from(error: MainError) -> Self {
        let context = format!("{}", error); // Capture the context string from the MainError
        MyTickError::MainError(Box::new(error), context)
    }
}
