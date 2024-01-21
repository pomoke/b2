#![no_main]
#![no_std]
#![cfg_attr(target_os = "uefi", feature(abi_efiapi))]
#![allow(stable_features)]
#![feature(once_cell)]
#![feature(result_flattening)]
extern crate alloc;
mod config;
mod console;
mod error;
mod graphics;
mod initedcell;
mod io;
mod linux;

use core::arch::asm;
use core::panic;
use once_cell::unsync::OnceCell;

use crate::console::efi::EFIConsole;
use crate::io::ReadOne;
use alloc::boxed::Box;
use alloc::vec;
use anyhow::{anyhow, Context};
use log::info;
use uefi::prelude::*;
use uefi::proto::console::gop::{BltOp, BltPixel, GraphicsOutput};
use uefi::proto::console::{gop, serial as efi_serial, text};
use uefi::proto::media::file::File;
use uefi::proto::media::file::FileAttribute;
use uefi::proto::media::file::FileInfo;
use uefi::CStr16;
use uefi::Result;
use uefi_services::{println, system_table};

#[cfg(not(all(target_arch = "x86_64", target_os = "uefi")))]
compile_error!("b2 only supports x86_64-unknown-uefi for now.");

//pub static mut system_table: OnceCell<SystemTable<Boot>> = OnceCell::new();

#[doc(hidden)]
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    println!("\nb2 has ran into some problem, system halted.");
    println!("panic: {}", info);
    println!("Please report this issue at github.com/pomoke/b2.");

    #[cfg(target_arch = "x86_64")]
    unsafe {
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

#[entry]
fn main(image_handle: Handle, mut st: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut st).unwrap();

    println!("b2 loader, version 0.0.1");
    info!("Vendor {}", st.firmware_vendor());
    info!("Version {}", st.firmware_revision());
    let rev = st.uefi_revision();
    let bs = st.boot_services();
    info!("EFI {}.{}", rev.major(), rev.minor());
    let mut root_protocol = bs.get_image_file_system(image_handle).unwrap();
    let mut rootfs = root_protocol.open_volume().unwrap();
    println!("{:?}", rootfs.get_boxed_info::<FileInfo>().unwrap());
    let mut file = rootfs
        .open(
            cstr16!("efi\\boot\\bootx64.efi"),
            uefi::proto::media::file::FileMode::Read,
            FileAttribute::READ_ONLY,
        )
        .unwrap();
    let mut file = file.into_regular_file().unwrap();
    let mut file_info: Box<FileInfo> = file.get_boxed_info().unwrap();
    println!("file size {}", file_info.file_size());
    let mut buf = vec![];
    buf.resize(1024 * 1024, 0);

    let size = file.read(buf.as_mut_slice()).unwrap();
    println!("read {} bytes", size);

    let mut console = EFIConsole::from_system_table();
    loop {
        println!("{:?}", console.read_one().unwrap());
    }

    /*
    if let Err(e) = draw(bs) {
        info!("error!");
    }
    */
    bs.stall(10_000_000);

    //system_table.runtime_services().reset(uefi::table::runtime::ResetType::Shutdown, Status(0), None);
    Status::SUCCESS
}
