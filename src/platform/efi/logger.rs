//! A simple logger logs to EFI variable.
//! Log can be extracted with other EFI programs or in an OS.
//!
//! NOTE: If you find any firmware stores volatile variables in NVRAM or other problems, please open an issue to report!

use alloc::{borrow::ToOwned, format, string::String, vec::Vec};
use log::{Level, LevelFilter, Metadata, Record};
use uefi::{
    cstr16,
    table::runtime::{VariableAttributes, VariableVendor},
    Status,
};
use uefi_services::{println, system_table};

use crate::platform::efi::{B2_UUID, B2_VENDOR};

const LOG_VARIABLE: &'static str = "Log";
const EFI_VAR_LOGGER: &EfiVarLogger = &EfiVarLogger {};

struct EfiVarLogger {}

impl log::Log for EfiVarLogger {
    #[cfg(debug_assertions)]
    fn enabled(&self, metadata: &Metadata) -> bool {
        // In crate `log`, more serious level has lower value.
        metadata.level() <= Level::Debug
    }

    #[cfg(not(debug_assertions))]
    fn enabled(&self, metadata: &Metadata) -> bool {
        // In crate `log`, more serious level has lower value.
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let st = system_table();
            let rs = st.runtime_services();
            let line = format!("[{}] {}\n", record.level(), record.args());

            // We try append write first.
            // If we got INVALID PARAMETER, then we need to read, then write.
            let ret = rs.set_variable(
                cstr16!("Log"),
                &B2_VENDOR,
                VariableAttributes::RUNTIME_ACCESS
                    | VariableAttributes::BOOTSERVICE_ACCESS
                    | VariableAttributes::APPEND_WRITE,
                line.as_bytes(),
            );
            if let Err(e) = ret {
                if e.status() == uefi::Status::INVALID_PARAMETER {
                    let ret = rs
                        .get_variable_boxed(cstr16!("Log"), &B2_VENDOR)
                        .map(|mut x| {
                            // This is safe, as we have cleared Log variable on log init.
                            unsafe {
                                String::from_raw_parts(x.0.as_mut_ptr(), x.0.len(), x.0.len())
                            }
                        })
                        .or_else(|e| match e.status() {
                            Status::NOT_FOUND => Ok("".to_owned()),
                            _ => Err(e),
                        })
                        .and_then(|s| {
                            let this_log = format!("{}{}", s, line);
                            rs.set_variable(
                                cstr16!("Log"),
                                &B2_VENDOR,
                                VariableAttributes::RUNTIME_ACCESS
                                    | VariableAttributes::BOOTSERVICE_ACCESS,
                                this_log.as_bytes(),
                            )
                        });
                }
            }
        }
    }

    fn flush(&self) {}
}

pub fn set_efi_var_logger() {
    // Clear log variable.
    let st = system_table();
    let rs = st.runtime_services();
    let header = String::from("b2 loader\n");
    // Delete variable.
    rs.set_variable(
        cstr16!("Log"),
        &B2_VENDOR,
        VariableAttributes::RUNTIME_ACCESS | VariableAttributes::BOOTSERVICE_ACCESS,
        &[],
    );
    log::set_logger(&EFI_VAR_LOGGER).unwrap();
    log::set_max_level(LevelFilter::Debug);
}
