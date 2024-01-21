//! UEFI Image Boot
//! Load a UEFI Image from given path, and jump to it.

use alloc::vec;
use alloc::{borrow::ToOwned, string::String, vec::Vec};
use anyhow::{anyhow, Context};
use log::{info, log, warn};
use uefi::proto::device_path::text::{AllowShortcuts, DisplayOnly};
use uefi::table::runtime::{ResetType, VariableAttributes, VariableVendor};
use uefi::{cstr16, Status};
use uefi::{
    proto::{
        device_path::{
            self,
            build::DevicePathBuilder,
            text::{DevicePathFromText, DevicePathToText},
            DevicePath, FfiDevicePath,
        },
        loaded_image::LoadedImage,
    },
    table::boot::LoadImageSource,
    CString16,
};
use uefi_services::{println, system_table};

use crate::io::file;
use crate::{io::file::File, platform::ToError};
use config::BootTarget;

use crate::platform::PlatformFile;

use crate::boot::boot::BootAble;

pub struct EFIBoot {
    path: String,
    cmdline: Option<String>,
    device: Option<String>,
}
impl EFIBoot {
    pub fn from_path(path: &str) -> Self {
        Self {
            path: path.to_owned(),
            cmdline: None,
            device: None,
        }
    }

    pub fn create(path: &str, cmdline: Option<&str>) -> Self {
        Self {
            path: path.to_owned(),
            cmdline: cmdline.map(ToOwned::to_owned),
            device: None,
        }
    }
}

impl BootAble for EFIBoot {
    /// Boot EFI Image.
    ///
    /// EFI does not know what is root directory.
    /// So it is required to get device manually.
    ///
    /// `Loaded Image` protocol is required to set rootfs and argv properly.
    ///
    fn boot(&mut self) -> anyhow::Result<!> {
        let st = unsafe { system_table() };
        let bs = st.boot_services();
        let device_path_from_text = bs
            .get_handle_for_protocol::<DevicePathFromText>()
            .core_err()?;
        let device_path_from_text = bs
            .open_protocol_exclusive::<DevicePathFromText>(device_path_from_text)
            .core_err()?;
        let this_image = bs.image_handle();
        let image_protocol = bs
            .open_protocol_exclusive::<LoadedImage>(this_image)
            .core_err()?;
        let root_device_handle = image_protocol.device().unwrap();
        let root_device = bs
            .open_protocol_exclusive::<DevicePath>(root_device_handle)
            .core_err()?;
        let device_path_to_text = bs
            .get_handle_for_protocol::<DevicePathToText>()
            .core_err()?;
        let device_path_to_text = bs
            .open_protocol_exclusive::<DevicePathToText>(device_path_to_text)
            .core_err()?;

        let file_path = CString16::try_from(self.path.as_str())
            .map_err(|_| anyhow!("Failed to convert"))
            .context("")?;
        let file_path = device_path_from_text
            .convert_text_to_device_path(&file_path)
            .core_err()
            .context("Failed to convert!")?;

        // We have to join two string of device path, as uefi-rs does not implement DevicePathUtil.
        let root_device = device_path_to_text
            .convert_device_path_to_text(
                bs,
                &root_device,
                DisplayOnly(false),
                AllowShortcuts(false),
            )
            .core_err()?;
        let file_device = device_path_to_text
            .convert_device_path_to_text(bs, file_path, DisplayOnly(false), AllowShortcuts(false))
            .core_err()?;
        let mut full_path = String::new();
        root_device
            .as_str_in_buf(&mut full_path)
            .map_err(|_| anyhow!("Failed to convert to rust string."))?;
        full_path.push('/');
        file_device
            .as_str_in_buf(&mut full_path)
            .map_err(|_| anyhow!("Failed to convert to rust string."))?;
        let full_path =
            CString16::try_from(full_path.as_str()).map_err(|_| anyhow!("failed to convert"))?;
        println!("{}", full_path);
        let full_path = device_path_from_text
            .convert_text_to_device_path(&full_path)
            .core_err()?;

        let image = bs
            .load_image(
                bs.image_handle(),
                LoadImageSource::FromDevicePath {
                    device_path: full_path,
                    from_boot_manager: false,
                },
            )
            .core_err()
            .context("failed to load image")?;
        // Configure load.
        let mut image_protocol = bs
            .open_protocol_exclusive::<LoadedImage>(image)
            .core_err()?;
        let config = CString16::try_from(self.cmdline.as_ref().unwrap_or(&"".to_owned()).as_str())
            .map_err(|_| anyhow!("Failed to convert!"))?;
        unsafe {
            image_protocol.set_load_options(config.as_ptr() as *const u8, config.num_bytes() as u32)
        };
        // Start image.
        bs.start_image(image).core_err()?;
        Err(anyhow!("unknown error. we successfully boot an image and returned. this should not happen for a program."))
    }
    fn load(&mut self) -> anyhow::Result<()> {
        todo!()
    }
}

pub struct Platform {}

pub fn boot(target: &BootTarget) -> anyhow::Result<bool> {
    match target {
        BootTarget::EFI { path, cmdline } => {
            let cmdline = cmdline.as_ref().map(|x| x.as_ref());
            let mut boot = EFIBoot::create(path, cmdline);
            boot.boot()?;
        }
        BootTarget::FirmwareSetup => {
            let st = system_table();
            let rs = st.runtime_services();
            let mut buf = [0u8; 8];
            rs.get_variable(
                cstr16!("OsIndicationsSupported"),
                &VariableVendor::GLOBAL_VARIABLE,
                &mut buf,
            )
            .map_err(|_| anyhow!("Failed to get variable of `OsIndicationsSupported`. Does firmware support this feature?"))?;
            if buf[0] & 1 != 1 {
                return Err(anyhow!(
                    "this device does not support reboot into firmware setup."
                ));
            }
            info!("reboot to fw ok!");
            let mut buf = [1u8, 0, 0, 0, 0, 0, 0, 0];
            println!("{:?}", rs.variable_keys().unwrap());
            rs.set_variable(
                cstr16!("OsIndications"),
                &VariableVendor::GLOBAL_VARIABLE,
                VariableAttributes::empty(),
                &buf,
            )
            .map_err(|e| anyhow!("Failed to set variable due to {}", e))?;
            rs.reset(ResetType::COLD, Status::SUCCESS, None);
        }
        BootTarget::Linux {
            kernel,
            initrd,
            cmdline,
        } => Err(anyhow!(
            "Loading Linux kernel directly is unavailable for now."
        )),
        BootTarget::Poweroff => {
            let st = system_table();
            let rs = st.runtime_services();
            rs.reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
        }
        BootTarget::Reboot => {
            let st = system_table();
            let rs = st.runtime_services();
            rs.reset(ResetType::COLD, Status::SUCCESS, None);
        }
        BootTarget::Debug => {
            let st = system_table();
            let bs = st.boot_services();
            let mmap_size = bs.memory_map_size();
            let (mmap_size, entry_size) = (mmap_size.map_size, mmap_size.entry_size);
            let max_size = mmap_size + 8 * entry_size;
            let mut mmap = vec![0u8; max_size];
            let memory_map = bs.memory_map(mmap.as_mut()).core_err()?;
            for i in memory_map.entries() {
                println!("{:?}: {}", i.ty, i.virt_start);
            }

            Ok(false)
        }
        BootTarget::Panic => {
            panic!("User requested panic.");
        }
        _ => {
            warn!("unknown or unsupported boot target {:?}.", target);
            Ok(false)
        }
    }
}
