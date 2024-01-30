extern crate alloc;
use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

/// The configuration of b2.
/// If config is unavailable or broken, b2 will simply load the fallback menu.
/// Config can be stored in EFI variable or files in %ESP for UEFI-based machines.
/// Config can be as well saved in NVRAM, a partition or in the bootloader.
#[derive(Debug, Clone)]
pub struct Config {
    /// Alternative title.
    name: Option<String>,
    /// Message
    message: Option<String>,
    /// Menu items.
    items: Vec<BootItem>,
    /// Default item, counts from 0.
    /// If the value is not in valid range, then first item (with id 0) will be selected.
    default: u32,
    /// Auto-boot timeout, in seconds ..
    /// `None` means the bootloader waits for user interaction.
    /// `Some(0)` means the bootloader will not show the menu and try to boot immediately. Users can hold `Shift` to show menu.
    timeout: Option<u32>,
    /// If this option is set, a password will be required for whole bootloader.
    password: Option<String>,
}

impl Config {
    fn fallback_menu() -> Self {
        Config {
            name: None,
            message: Some("Error: No valid config found.".to_owned()),
            items: vec![
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
            ],
            default: 0,
            timeout: None,
            password: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BootItem {
    name: String,
    target: BootTarget,
}

/// Boot Target - represents a bootable target.
///
#[derive(Debug, Clone)]
pub enum BootTarget {
    /// EFI image.
    #[cfg(target_os = "uefi")]
    EFI { path: String },
    /// Linux image.
    Linux {
        kernel: ImageLocation,
        initrd: Vec<ImageLocation>,
        cmdline: String,
    },
    /// Display a message. Short messages only.
    Message(String),
    /// On supported platforms, system will reboot to firmware setup.
    FirmwareSetup,
    /// Simply reboot the machine.
    Reboot,
    /// Try to poweroff. If failed, the bootloader will continue to run.
    Poweroff,
    /// On supported platforms, b2 will exit to run other bootable targets.
    Exit,
}

#[derive(Debug, Clone)]
pub enum ImageLocation {
    /// A path in filesystem. EFI only.
    Path(String),
    /// A whole partition as image.
    /// TODO: Auto-detection of single image and compound partition.
    Partition { disk: i32, part: i32 },
    /// Offsets in a partition.
    /// Use ioctl(FIBMAP) to get offsets of kernel image.
    Segments {
        disk: i32,
        part: i32,
        segments: Vec<Extent>,
    },
}

#[derive(Debug, Clone)]
pub struct Extent {
    from: u32,
    length: u32,
}
