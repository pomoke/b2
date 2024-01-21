use alloc::string::String;
use alloc::vec;
use anyhow::anyhow;
use anyhow::{Context, Result};

use super::console::{AcceleratorKey, Key};
pub trait Handle: Sized {
    /// Close current resource.
    /// It's preferred to release resource in `trait drop`.
    fn close(self) {
        drop(self)
    }
}

pub trait Stream {
    fn read(&mut self, buf: &mut [u8]) -> Result<i32>;
    fn write(&mut self, buf: &[u8]) -> Result<i32>;
}

pub trait Read<T> {
    fn read(&mut self, buf: &mut [T]) -> Result<i32>;
}

/// Read one item of T.
pub trait ReadOne<T> {
    fn read_one(&mut self) -> Result<T>;
}

/// Read a line from terminal.
///  
/// Note: This trait is not for reading password.
pub trait ReadString {
    /// Read a line, with echo.
    ///
    /// Do not use this function for secrets!
    /// Types of `Read<Key` will automatically implement this trait.
    fn read_line(&mut self, buf: &mut String) -> Result<i32>;
}

/// Read a line of secret (such as password) from terminal.
///
/// Input should be hidden or masked.
pub trait ReadSecret {
    /// Read a line, secretly and with no echo at all.
    ///
    /// For those who don't use *nix, this may cause confusion.
    fn read_pass(&mut self, buf: &mut String) -> Result<i32>;

    /// Same as read_pass(), which input is masked with `*`
    ///
    /// Common practice in most UI, but not commonly used in *nix.
    fn read_pass_star(&mut self, buf: &mut String) -> Result<i32>;
}

impl<T: ReadOne<Key> + core::fmt::Write> ReadString for T {
    fn read_line(&mut self, buf: &mut String) -> Result<i32> {
        let mut n = 0;
        loop {
            let i = self.read_one()?;
            match i {
                Key::Printable(k) => {
                    n += 1;
                    buf.push(k);
                    self.write_char(k)
                        .map_err(|_| anyhow!("Failed to write to console."))?;
                }
                Key::Accelerator(AcceleratorKey::Enter) => {
                    self.write_str(if cfg!(target_os = "uefi") {
                        "\r\n"
                    } else {
                        "\n"
                    })
                    .map_err(|_| anyhow!("Failed to write to console."))?;
                    return Ok(n);
                }
                _ => {}
            }
        }
    }
}

pub trait Write<T> {
    fn write(&mut self, buf: &[T]) -> Result<i32>;
}

pub trait WriteString {
    fn write_str(&mut self, buf: &str) -> Result<i32>;
}

/// Block device meta-interface.
///
/// Note: this trait only support offset within 2^31-1 (2GB). This should be enough for bootloaders.
pub trait BlockDevice: Read<u8> {
    fn get_pos(&self) -> Result<i32>;
    fn set_pos(&mut self, pos: i32) -> Result<i32>;
}
