//! UEFI File

use alloc::{borrow::ToOwned, vec::Vec};
use anyhow::{anyhow, Context, Result};
use uefi::{
    cstr16,
    proto::media::file::{File as BaseFile, FileAttribute, FileHandle, FileInfo, FileMode},
    CStr16, CString16,
};
use uefi_services::system_table;

use crate::platform::ToError;

use crate::io::file::File;

pub struct EFIFile {
    file: FileHandle,
}

impl File<EFIFile> {
    pub fn open(path: &str) -> Result<Self> {
        let st = unsafe { system_table() };
        let bs = st.boot_services();
        let image_handle = bs.image_handle();
        let mut root_protocol = bs.get_image_file_system(image_handle).core_err()?;
        let mut rootfs = root_protocol.open_volume().core_err()?;
        let path = CString16::try_from(path).map_err(|_| anyhow!("Failed to convert!"))?;
        let file = rootfs
            .open(&path, FileMode::Read, FileAttribute::READ_ONLY)
            .core_err()
            .context("Failed to open!")?;
        Ok(Self {
            backend: EFIFile { file },
        })
    }

    /// Leak the EFI file handle inside.
    pub fn leak(self) -> FileHandle {
        self.backend.file
    }

    pub fn from_efi_handle(h: FileHandle) -> Result<Self> {
        Ok(Self {
            backend: EFIFile { file: h },
        })
    }

    pub fn read_all(self) -> Result<Vec<u8>> {
        let mut file = self
            .backend
            .file
            .into_regular_file()
            .ok_or_else(|| anyhow!("Not a file."))?;
        let info = file.get_boxed_info::<FileInfo>().core_err()?;
        let size = info.file_size();
        let mut buf = Vec::new();
        buf.try_reserve(size as usize).core_err()?;
        buf.resize(size as usize, 0);
        file.read(&mut buf).core_err()?;
        Ok(buf)
    }
}
