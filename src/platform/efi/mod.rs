use uefi::{guid, table::runtime::VariableVendor, Guid};

pub mod boot;
pub mod console;
pub mod efi_error;
pub mod entry;
pub mod file;
pub mod init;
pub mod logger;
pub mod tty;

pub const B2_UUID: Guid = guid!("95f342d7-c48a-4799-8df5-6710597a7430");
pub const B2_VENDOR: VariableVendor = VariableVendor(B2_UUID);
