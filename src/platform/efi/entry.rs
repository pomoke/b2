//! Actual code is placed here.

use core::arch::asm;

//use crate::console::efi::EFIConsole;
use crate::io::{LineEdit, ReadSecret, ReadString};
use crate::platform::efi::console::EFIConsole;
use crate::platform::efi::B2_UUID;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;

use anyhow::Context;
use log::info;
use uefi::prelude::*;
use uefi::proto::console::gop::{BltOp, BltPixel, GraphicsOutput};

use crate::boot::boot::BootAble;
use crate::console::basicmenu::BasicMenu;
use crate::platform::efi::boot::EFIBoot;
use crate::platform::efi::efi_error::ToError;
use crate::platform::efi::file::EFIFile;
use config::Config;
use uefi::Result;
use uefi_services::println;

#[doc(hidden)]
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    println!("\nb2 has ran into some problem, system halted.");
    println!("panic: {}\n", info);
    println!("If you believe this is some sort of bug, report at github.com/pomoke/b2 .");

    #[cfg(target_arch = "x86_64")]
    unsafe {
        // HLT is introduced since 8086. It's always OK to call this.
        asm!("hlt", options(nomem, nostack));
    };

    loop {}
}

fn draw(bs: &BootServices) -> Result {
    let gop_handle = bs.get_handle_for_protocol::<GraphicsOutput>()?;
    let mut gop = bs.open_protocol_exclusive::<GraphicsOutput>(gop_handle)?;
    let mode_info = gop.current_mode_info();
    let width = mode_info.resolution().0;
    let height = mode_info.resolution().1;

    let op = BltOp::VideoFill {
        color: BltPixel::new(255, 0, 0),
        dest: (0, 0),
        dims: (width, height),
    };
    gop.blt(op)?;
    bs.stall(5_000_000);
    let op = BltOp::VideoFill {
        color: BltPixel::new(0, 255, 0),
        dest: (0, 0),
        dims: (width, height),
    };
    gop.blt(op)?;
    bs.stall(5_000_000);
    let op = BltOp::VideoFill {
        color: BltPixel::new(0, 0, 255),
        dest: (0, 0),
        dims: (width, height),
    };
    gop.blt(op)?;
    bs.stall(5_000_000);
    info!(
        "Screen: {}x{}",
        mode_info.resolution().0,
        mode_info.resolution().1
    );
    Ok(())
}

#[cfg(target_os = "uefi")]
#[entry]
pub fn main(image_handle: Handle, mut st: SystemTable<Boot>) -> Status {
    use anyhow::anyhow;
    use uefi::{guid, table::runtime::VariableVendor};

    use crate::{io::file::File, config::{BootConfig, do_boot}};

    use config::{BootOption, BootOptionItem, BootOptionKind};

    uefi_services::init(&mut st).unwrap();
    let rev = st.uefi_revision();
    let bs = st.boot_services();
    let rs = st.runtime_services();

    info!("FW Vendor {}", st.firmware_vendor());
    info!("FW Version {}", st.firmware_revision());
    info!("EFI {}.{}", rev.major(), rev.minor());
    println!("b2 loader, version 0.0.2");

    //bs.set_watchdog_timer(0, 0, None).core_err().context("Failed to stop watchdog.").unwrap();

    let mut config: BootConfig =
        File::<EFIFile>::open("b2.conf")
            .and_then(|x| x.read_all())
            .or_else(|_| {
                rs.get_variable_boxed(cstr16!("Config"), &VariableVendor(B2_UUID))
                    .core_err()
                    .map(|x| x.0.into_vec())
            })
            .and_then(|x| {
                serde_json_core::from_slice::<Config>(x.as_slice())
                    .map_err(|x| anyhow!("{}", x))
            })
            .map(|x| BootConfig(x.0))
            .inspect_err(|e| println!("error loading config: {:?}", e))
            .unwrap_or(BootConfig::fallback_menu());

    let mut console = EFIConsole::from_system_table();
    let boot_config = BootConfig::fallback_menu();
    let mut buf = String::new();
    //console.edit_line(&mut buf, "test: ").unwrap();

    // Test config menu.
    let bootconf_test = [
        BootOption {
            identifier: String::from("NOACPI"),
            pos: 0,
            name: String::from("Disable ACPI"),
            description: None,
            option: BootOptionKind::Bool(String::from("acpi=no")),
        },
        BootOption {
            identifier: String::from("ROOT"),
            pos: 0,
            name: String::from("Boot with"),
            description: None,
            option: BootOptionKind::Multiple(vec![
                BootOptionItem {
                    identifier: String::from("ROOT1"),
                    name: String::from("debian"),
                    description: None,
                    value: String::from("root=/dev/sda1"),
                },
                BootOptionItem {
                    identifier: String::from("ROOT2"),
                    name: String::from("arch"),
                    description: None,
                    value: String::from("root=/dev/sda2"),
                },
                BootOptionItem {
                    identifier: String::from("ROOT3"),
                    name: String::from("nix"),
                    description: None,
                    value: String::from("root=/dev/sda3"),
                },
            ]),
        },
        BootOption {
            identifier: String::from("TEST"),
            pos: 0,
            name: String::from("test"),
            description: None,
            option: BootOptionKind::Template(String::from("what=")),
        },
    ];

    let menu = BasicMenu {};
    //menu.boot_config(&bootconf_test, &mut console).unwrap();

    loop {
        let option = menu.prompt(&boot_config.0, &mut console).unwrap();
        println!("{:?}", option);
        let boot_result = do_boot(&option.target);
        match boot_result {
            // Ok() variant is used to indicate exit without error.
            Ok(true) => break,
            Ok(_) => (),
            Err(e) => {
                println!("Failed to boot: {}", e);
            }
        }
    }

    /*
    if let Err(e) = draw(bs) {
        info!("error!");
    }
    */
    //bs.stall(10_000_000);
    println!("bye!");

    //system_table.runtime_services().reset(uefi::table::runtime::ResetType::Shutdown, Status(0), None);
    Status::SUCCESS
}
