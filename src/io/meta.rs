use alloc::vec;
use alloc::{string::String, vec::Vec};
use anyhow::Result;

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

/// Read one item of T at once.
pub trait ReadOne<T> {
    fn read_one(&mut self) -> Result<T>;
}

pub trait ReadString {
    fn read_str(&mut self, buf: &mut String) -> Result<i32>;
}

impl<T: Read<Key>> ReadString for T {
    fn read_str(&mut self, buf: &mut String) -> Result<i32> {
        let mut n = 0;
        loop {
            let mut keys = vec![];
            let k = self.read(&mut keys)?;
            for i in keys {
                match i {
                    Key::Printable(k) => {
                        n += 1;
                        buf.push(k);
                    }
                    Key::Accelerator(AcceleratorKey::Enter) => return Ok(n),
                    _ => {}
                }
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
pub trait Block: Read<u8> {
    fn get_pos(&self) -> Result<i32>;
    fn set_pos(&mut self, pos: i32) -> Result<i32>;
}
