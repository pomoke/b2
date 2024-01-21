/// Boot Image Loader.
pub mod boot;
#[cfg(target_os = "uefi")]
pub mod efi;
pub mod linux;
