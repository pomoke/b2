#![allow(unused)]

extern crate alloc;
use core::fmt::Display;

use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use anyhow::anyhow;
use config::boot::Config;
use config::BootItem;
use config::BootTarget;
use log::info;
use serde::Deserialize;
use serde::Serialize;

#[cfg(target_os = "uefi")]
use crate::boot::boot::BootAble;
#[cfg(target_os = "uefi")]
use crate::platform::efi::boot::boot as platform_boot;
#[cfg(target_os = "uefi")]
use crate::platform::efi::boot::EFIBoot;
#[cfg(target_os = "uefi")]
use crate::platform::efi::efi_error::ToError;
#[cfg(target_os = "uefi")]
use crate::platform::println;

pub(crate) mod boot_config;
pub(crate) mod lock;

pub struct BootConfig(pub Config);

impl BootConfig {
    pub fn fallback_menu() -> Self {
        BootConfig(Config {
            name: Some("b2 Menu".to_owned()),
            message: Some("".to_owned()),
            items: vec![
                #[cfg(target_os = "uefi")]
                BootItem {
                    name: "Linux".to_owned(),
                    target: BootTarget::EFI {
                        path: "/linux/vmlinuz".to_owned(),
                        cmdline: Some("initrd=\\linux\\initrd.gz".to_owned()),
                    },
                },
                BootItem {
                    name: "Reboot".to_owned(),
                    target: BootTarget::Reboot,
                },
                BootItem {
                    name: "Poweroff".to_owned(),
                    target: BootTarget::Poweroff,
                },
                BootItem {
                    name: "Firmware Setup".to_owned(),
                    target: BootTarget::FirmwareSetup,
                },
                BootItem {
                    name: "Exit".to_owned(),
                    target: BootTarget::Exit,
                },
                BootItem {
                    name: "Debug Info".to_owned(),
                    target: BootTarget::Debug,
                },
                #[cfg(debug_assertions)]
                BootItem {
                    name: "Panic".to_owned(),
                    target: BootTarget::Panic,
                },
            ],
            default: 0,
            timeout: None,
            password: None,
        })
    }
}

pub fn do_boot(a: &BootTarget) -> anyhow::Result<bool> {
    match a {
        BootTarget::Exit => Ok(true),
        BootTarget::Message(msg) => {
            println!("{}", msg);
            Ok(false)
        }
        BootTarget::Nop => Ok(false),
        _ => platform_boot(a),
    }
}
