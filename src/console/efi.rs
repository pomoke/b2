//! EFI Based Console - for use with EFI Text Terminal.

use core::fmt::Write;

use alloc::{borrow::ToOwned, vec::Vec};
use anyhow::{anyhow, Context, Result};
use uefi::{
    prelude::BootServices,
    proto::console::text::{Input, Output},
    table::{boot::ScopedProtocol, SystemTable, Boot},
    Event,
};
use uefi_services::system_table;

use crate::{
    error::ToError,
    io::{
        console::{Console, Key},
        ReadOne,
    },
};

/// UEFI text protocol based console.
pub struct EFIConsole<'a> {
    input: &'a mut Input,
    output: &'a mut Output<'a>,
    boot_service: &'a BootServices,
}

impl<'a> EFIConsole<'a> {
    pub fn new(input: &'a mut Input,output: &'a mut Output<'a>,boot_service: &'a BootServices) -> Self {
        Self {
            input,
            output,
            boot_service
        }
    }

    pub fn from_system_table() -> Self {
        let st = unsafe {system_table().as_mut()};
        let st2 = unsafe {system_table().as_mut()};
        let st3 = unsafe {system_table().as_mut()};
        let boot_service = st2.boot_services();
        let input = st.stdin();
        let output = st3.stdout();
        Self {
            input,output,boot_service
        }
    }
}

impl<'a> Write for EFIConsole<'a> {
    /*
    fn write_fmt(mut self: &mut Self, args: core::fmt::Arguments<'_>) -> core::fmt::Result {
       self.output.write_fmt(args)
    }
    */
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
            .map(|x| x.ok_or_else(|| anyhow!("Failed to read key.")))
            .flatten()
            .context("Failed to read key.")?;
        Ok(key.into())
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
