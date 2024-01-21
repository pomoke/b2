//! EFI Based Console - for use with EFI Text Terminal.

use core::fmt::Write;

use alloc::{borrow::ToOwned, string::String, vec::Vec};
use anyhow::{anyhow, Context, Result};
use uefi::{
    prelude::BootServices,
    proto::console::text::{Input, Output},
    table::{boot::ScopedProtocol, Boot, SystemTable},
    Event,
};
use uefi_services::system_table;

use crate::{
    error::ToError,
    io::{
        console::{AcceleratorKey, Console, Key},
        ReadOne, ReadSecret,
    },
};

/// UEFI text protocol based console.
pub struct EFIConsole<'a> {
    input: &'a mut Input,
    output: &'a mut Output,
    boot_service: &'a BootServices,
}

impl<'a> EFIConsole<'a> {
    pub fn new(
        input: &'a mut Input,
        output: &'a mut Output,
        boot_service: &'a BootServices,
    ) -> Self {
        Self {
            input,
            output,
            boot_service,
        }
    }

    /// Acquire current EFI terminal.
    pub fn from_system_table() -> Self {
        let st = unsafe { system_table().as_mut() };
        let st2 = unsafe { system_table().as_mut() };
        let st3 = unsafe { system_table().as_mut() };
        let boot_service = st.boot_services();
        let input = st2.stdin();
        let output = st3.stdout();
        Self {
            input,
            output,
            boot_service,
        }
    }
}

impl<'a> Write for EFIConsole<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.output.write_str(s)
    }
}

impl<'a> ReadOne<Key> for EFIConsole<'a> {
    fn read_one(&mut self) -> Result<Key> {
        let event = self.input.wait_for_key_event();
        let event = unsafe { event.unsafe_clone() };
        self.boot_service
            .wait_for_event(&mut [event][..])
            .core_err()
            .context("Failed to wait for event.")?;

        let key = self
            .input
            .read_key()
            .core_err()
            .and_then(|x| x.ok_or_else(|| anyhow!("No key value!")))
            .context("Failed to read key.")?;

        Ok(key.into())
    }
}

impl<'a> ReadSecret for EFIConsole<'a> {
    fn read_pass(&mut self, buf: &mut String) -> Result<i32> {
        let mut n = 0;
        loop {
            let i = self.read_one()?;
            match i {
                Key::Printable(k) => {
                    n += 1;
                    buf.push(k);
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
    fn read_pass_star(&mut self, buf: &mut String) -> Result<i32> {
        let mut n = 0;
        loop {
            let i = self.read_one()?;
            match i {
                Key::Printable(k) => {
                    n += 1;
                    buf.push(k);
                    self.write_char('*')
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

impl<'a> Console for EFIConsole<'a> {
    fn get_cursor(&mut self) -> Result<()> {
        todo!()
    }

    fn read_key(&mut self) -> Result<Option<Key>> {
        todo!()
    }

    fn set_cursor(&mut self) -> Result<()> {
        todo!()
    }

    fn wait_for_key(&mut self) -> Result<Key> {
        todo!()
    }
}
