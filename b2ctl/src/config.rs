use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Alternative title.
    pub(crate) name: Option<String>,
    /// Message
    pub(crate) message: Option<String>,
    /// Menu items.
    pub(crate) items: Vec<BootItem>,
    /// Default item, counts from 0.
    /// If the value is not in valid range, then first item (with id 0) will be selected.
    pub(crate) default: u32,
    /// Auto-boot timeout, in seconds.
    ///
    /// Special values:
    /// * `None`: the bootloader waits for user interaction.
    /// * `Some(0)`: the bootloader will not show the menu and try to boot immediately. Press any key to show menu.
    pub(crate) timeout: Option<u32>,
    /// If this option is set, a password will be required for whole bootloader.
    pub(crate) password: Option<String>,
}

impl Config {
    pub fn fallback_menu() -> Self {
        Config {
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
            ],
            default: 0,
            timeout: None,
            password: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootItem {
    pub name: String,
    pub(crate) target: BootTarget,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ImageLocation {
    Path(String),
    /// A whole partition as image.
    /// TODO: Auto-detection of single image and compound partition.
    Partition {
        disk: i32,
        part: i32,
    },
    /// Offsets in a partition.
    /// Use ioctl(FIBMAP) to get offsets of kernel image.
    Segments {
        disk: i32,
        /// 0 is for whole disk.
        part: i32,
        /// No support for non-continous file for now.
        segment: Extent,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Extent {
    from: u32,
    length: u32,
}
