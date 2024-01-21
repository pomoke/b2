//! UEFI Image Boot
//! Load a UEFI Image from given path, and jump to it.

use alloc::vec;
use alloc::{borrow::ToOwned, string::String, vec::Vec};
use anyhow::{anyhow, Context};
use uefi::proto::device_path::text::{AllowShortcuts, DisplayOnly};
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
use crate::{
    error::ToError,
    io::file::{efi::EFIFile, File},
};

use super::boot::BootAble;

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
}
impl BootAble for EFIBoot {
    /// Boot EFI Image.
    ///
    /// EFI does not know what is / .
    /// It is required to get device manually.
    ///
    /// Loaded image protocol is required to set rootfs and argv properly.
    fn boot(&mut self) -> anyhow::Result<!> {
        let st = unsafe { system_table().as_ref() };
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
        let root_device_handle = image_protocol.device();
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
            .convert_device_path_to_text(bs, &file_path, DisplayOnly(false), AllowShortcuts(false))
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
                LoadImageSource::FromFilePath {
                    file_path: full_path,
                    from_boot_manager: false,
                },
            )
            .core_err()
            .context("failed to load image")?;
        // Configure load.
        let mut image_protocol = bs
            .open_protocol_exclusive::<LoadedImage>(image)
            .core_err()?;
        let mut config = CString16::try_from(self.cmdline.as_ref().unwrap_or(&"initrd=\\linux\\initrd.gz".to_owned()).as_str()).map_err(|_| anyhow!("Failed to convert!"))?;
        unsafe { image_protocol.set_load_options(config.as_ptr() as *const u8, config.num_bytes() as u32) };
        // Start image.
        bs.start_image(image).core_err()?;
        Err(anyhow!("Unknown error."))
    }
    fn new() -> Self {
        Self {
            path: "".to_owned(),
            cmdline: None,
            device: None,
        }
    }
    fn load(&mut self) -> anyhow::Result<()> {
        todo!()
    }
}
