#![no_main]
#![no_std]
#![cfg_attr(target_os = "uefi", feature(abi_efiapi))]
#![allow(stable_features)]
#![feature(error_in_core)]
#![feature(once_cell)]
#![feature(result_flattening)]
#![feature(exclusive_range_pattern)]
#![feature(is_some_and)]
#![feature(never_type)]
#![feature(cfg_match)]

// b2 has not been tested on architectures other than x86_64.

#[cfg(not(target_os = "uefi"))]
compile_error!("b2 only supports EFI targets for now.");

extern crate alloc;
mod boot;
pub mod config;
mod console;
mod graphics;
mod initedcell;
mod io;
mod platform;
pub mod error;

// This file does not contain actual entrypoint.
//
// Check `platform/<firmware-type>/entry.rs` for firmware specific entrypoint.
