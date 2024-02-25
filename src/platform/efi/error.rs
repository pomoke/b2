use alloc::string::String;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EfiError {
    #[error("tried boot, but returned properly.")]
    BootReturned,
    #[error("failed to boot, efi error {0}")]
    BootError(uefi::Error),
    #[error("Variable {0} does not exist.")]
    NoEfiVariable(String),
    #[error("Failed to set variable {1}, reason {0}")]
    WriteEfiVariable(uefi::Error, String),
    #[error("Failed to get variable {1}, reason {0}")]
    GetEfiVariable(uefi::Error, String),
    #[error("Failed to open protocol, efi error {0}")]
    OpenProtocol(uefi::Error),
}
