#[cfg(target_os = "uefi")]
pub mod efi;

#[cfg(target_os = "uefi")]
pub use {efi::efi_error::ToError, uefi_services::println};

#[cfg(target_os = "uefi")]
type PlatformFile = efi::file::EFIFile;

#[cfg(target_os = "uefi")]
type PlatformConsole = efi::console::EFIConsole;
