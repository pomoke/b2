#![allow(unused)]

extern crate alloc;
use core::fmt::Display;

#[cfg(feature = "no_std")]
use alloc::{borrow::ToOwned, string::String, vec, vec::Vec};

use serde::{Deserialize, Serialize};

/// The configuration of b2.
/// If config is unavailable or broken, b2 will simply load the fallback menu.
/// Config can be stored in EFI variable or files in %ESP on UEFI-based machines.
/// Config can be as well saved in NVRAM, a partition or in the bootloader.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Alternative title.
    pub name: Option<String>,
    /// Message
    pub message: Option<String>,
    /// Menu items.
    pub items: Vec<BootItem>,
    /// Default item, counts from 0.
    /// If the value is not in valid range, then first item (with id 0) will be selected.
    pub default: u32,
    /// Auto-boot timeout, in seconds.
    ///
    /// Special values:
    /// * `None`: the bootloader waits for user interaction.
    /// * `Some(0)`: the bootloader will not show the menu and try to boot immediately. Press any key to show menu.
    pub timeout: Option<u32>,
    /// If this option is set, a password will be required for whole bootloader.
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootItem {
    pub name: String,
    pub target: BootTarget,
}

/// Boot Target - represents a bootable target.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BootTarget {
    /// EFI image.
    #[serde(rename = "efi")]
    EFI {
        path: String,
        cmdline: Option<String>,
    },
    /// Linux image.
    #[serde(rename = "linux")]
    Linux {
        kernel: ImageLocation,
        initrd: Vec<ImageLocation>,
        cmdline: String,
    },
    /// Display a message. Message should not be more than a page.
    #[serde(rename = "message")]
    Message(String),
    /// On supported platforms, system will reboot to firmware setup.
    #[serde(rename = "firmware_setup")]
    FirmwareSetup,
    /// Simply reboot the machine.
    #[serde(rename = "reboot")]
    Reboot,
    /// Try to poweroff. If failed, the bootloader will continue to run.
    #[serde(rename = "poweroff")]
    Poweroff,
    /// On supported platforms, b2 will exit to run other firmware bootable targets.
    #[serde(rename = "exit")]
    Exit,
    /// Show debug information.
    #[serde(rename = "debug")]
    Debug,
    /// Nothing happens.
    #[serde(rename = "nop")]
    Nop,
    /// Bootloader will panic. Only available in debug builds.
    #[serde(rename = "panic")]
    Panic,
    /// Unknown.
    #[serde(rename = "unknown")]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ImageLocation {
    /// A path in file system.
    Path(String),
    /// A whole partition as image.
    /// TODO: Auto-detection of single image and compound partition.
    Partition { disk: i32, part: i32 },
    /// Offsets in a partition.
    /// Use ioctl(FIBMAP) to get offsets of kernel image.
    Segments {
        disk: i32,
        /// 0 is for whole disk.
        part: i32,
        /// No support for non-continous file for now.
        segment: Extent,
    },
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extent {
    from: u32,
    length: u32,
}
