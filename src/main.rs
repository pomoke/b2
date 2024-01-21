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

extern crate alloc;
mod boot;
mod config;
mod console;
mod error;
mod graphics;
mod initedcell;
mod io;

use core::arch::asm;

use crate::console::efi::EFIConsole;
use crate::io::{ReadSecret, ReadString};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec;

use anyhow::Context;
use log::info;
use uefi::prelude::*;
use uefi::proto::console::gop::{BltOp, BltPixel, GraphicsOutput};

use uefi::proto::media::file::File;
use uefi::proto::media::file::FileAttribute;
use uefi::proto::media::file::FileInfo;

use crate::boot::boot::BootAble;
use crate::boot::efi::EFIBoot;
use crate::error::ToError;
use uefi::Result;
use uefi_services::println;

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
    //bs.set_watchdog_timer(0, 0, None).core_err().context("Failed to stop watchdog.").unwrap();
    info!("EFI {}.{}", rev.major(), rev.minor());
    let mut root_protocol = bs
        .get_image_file_system(image_handle)
        .expect("No root fs specified. b2 will not work.");
    let mut rootfs = root_protocol.open_volume().unwrap();
    println!("{:?}", rootfs.get_boxed_info::<FileInfo>().unwrap());
    let file = rootfs
        .open(
            cstr16!("\\efi\\boot\\bootx64.efi"),
            uefi::proto::media::file::FileMode::Read,
            FileAttribute::READ_ONLY,
        )
        .unwrap();
    let mut file = file.into_regular_file().unwrap();
    let file_info: Box<FileInfo> = file.get_boxed_info().unwrap();
    println!("file size {}", file_info.file_size());
    let mut buf = vec![];
    buf.resize(1024 * 1024, 0);

    let size = file.read(buf.as_mut_slice()).unwrap();
    println!("read {} bytes", size);
    drop(root_protocol);
    drop(rootfs);
    drop(file);
    drop(file_info);

    let mut console = EFIConsole::from_system_table();
    let mut boot = EFIBoot::from_path("/linux/vmlinuz");
    println!("こんにちは");
    println!("你好");
    println!("Jó napot");

    loop {
        let mut s = String::new();
        console.read_line(&mut s).unwrap();
        println!("echo: {}", s);
        if s == "boot" {
            boot.boot().unwrap();
        }
        s.clear();
        console.read_pass(&mut s).unwrap();
        println!("echo: {}", s);
        s.clear();
        console.read_pass_star(&mut s).unwrap();
        println!("echo: {}", s);
        s.clear();
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
