use thiserror::Error;
use crate::platform::PlatformError;

#[derive(Debug, Error)]
pub enum B2Error {
    #[error("unimplemented")]
    Todo,
    #[error("Failed to convert to")]
    Conversion,
    #[error("Failed to read NVRAM")]
    ReadNVRAM,
    #[error("Failed to write to NVRAM")]
    WriteNVRAM,
    #[error("Index out of range")]
    OutOfRange,
    #[error("Failed to format")]
    Format,
    #[error("Unknown error")]
    Unknown,
    #[error("not a file")]
    NotFile,
    #[error(transparent)]
    PlatformError(PlatformError),
}