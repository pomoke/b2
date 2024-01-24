use config::{BootItem, BootTarget, Config};
use serde::{Deserialize, Serialize};

pub fn fallback_menu() -> Config {
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
