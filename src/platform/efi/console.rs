//! EFI Based Console - for use with EFI Text Terminal.

use core::{borrow::BorrowMut, fmt::Write};

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
    io::{
        console::{AcceleratorKey, Console, CursorStyle, Key, TerminalInfo},
        ReadOne, ReadSecret,
    },
    platform::efi::efi_error::ToError,
};

/// UEFI text protocol based console.
pub struct EFIConsole {}

impl EFIConsole {
    pub fn new() -> Self {
        Self {}
    }

    /// Acquire current EFI terminal.
    pub fn from_system_table() -> Self {
        Self {}
    }
}

impl Write for EFIConsole {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let mut st = system_table();
        let output = st.stdout();
        output.write_str(s)
    }
}

impl ReadOne<Key> for EFIConsole {
    fn read_one(&mut self) -> Result<Key> {
        let mut st = system_table();
        let mut input = st.stdin();
        let event = input
            .wait_for_key_event()
            .ok_or(anyhow!("failed to read key."))?;
        let event = unsafe { event.unsafe_clone() };
        system_table()
            .boot_services()
            .wait_for_event(&mut [event][..])
            .core_err()
            .context("Failed to wait for event.")?;

        let key = input
            .read_key()
            .core_err()
            .and_then(|x| x.ok_or_else(|| anyhow!("No key value!")))
            .context("Failed to read key.")?;

        Ok(key.into())
    }
}

impl<'a> ReadSecret for EFIConsole {
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

impl Console for EFIConsole {
    fn get_cursor(&mut self) -> Result<(i32, i32)> {
        let mut st = system_table();
        let stdout = st.stdout();
        let (x, y) = stdout.cursor_position();
        Ok((x as i32, y as i32))
    }

    fn set_cursor(&mut self, x: i32, y: i32) -> Result<()> {
        let mut st = system_table();
        let stdout = st.stdout();
        stdout
            .set_cursor_position(x as usize, y as usize)
            .core_err()?;
        Ok(())
    }

    fn set_cursor_style(&mut self, style: &crate::io::console::CursorStyle) -> Result<()> {
        let mut st = system_table();
        let stdout = st.stdout();
        stdout
            .enable_cursor(!matches!(style, CursorStyle::None))
            .or_else(|x| {
                if x.status() == uefi::Status::UNSUPPORTED {
                    Ok(())
                } else {
                    Err(x)
                }
            })
            .core_err()?;
        Ok(())
    }

    fn reset(&mut self) -> Result<()> {
        let mut st = system_table();
        let mut stdin = st.stdin();
        let mut st = system_table();
        let mut stdout = st.stdout();
        let mut st = system_table();
        let mut stderr = st.stderr();
        stdout.reset(true).core_err()?;
        stderr.reset(true).core_err()?;
        stdin.reset(true).core_err()?;
        Ok(())
    }

    fn terminal_info(&mut self) -> Result<TerminalInfo> {
        let mut st = system_table();
        let mut stdout = st.stdout();
        let mode = stdout
            .current_mode()
            .core_err()?
            .ok_or_else(|| anyhow!("unknown efi console mode."))?;
        Ok(TerminalInfo {
            size: (mode.columns(), mode.rows()),
        })
    }
}
