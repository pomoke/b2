use uefi::{guid, Guid};

pub mod boot;
pub mod console;
pub mod efi_error;
pub mod entry;
pub mod file;
pub mod init;
pub mod tty;

pub(crate) const B2_UUID: Guid = guid!("95f342d7-c48a-4799-8df5-6710597a7430");
