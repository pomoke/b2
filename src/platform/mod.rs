#[cfg(target_os = "uefi")]
pub mod efi;

#[cfg(target_os = "uefi")]
pub use {efi::efi_error::ToError, uefi_services::println};

#[cfg(target_os = "uefi")]
pub type PlatformFile = efi::file::EFIFile;

#[cfg(target_os = "uefi")]
pub type PlatformConsole = efi::console::EFIConsole;

// PlatformError contains a lifetime for use with error message.

#[cfg(target_os = "uefi")]
pub type PlatformError = efi::error::EfiError;
