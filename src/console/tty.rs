//! Serial-Based Console
//! The terminal requires a transport and a control implementation.
//! That is, you can use it for any stream based and in-band controlled console.

use crate::console::console::ConsoleStyle;
use crate::io::ReadOne;
use crate::io::{
    console::{Console, Key},
    Read,
};
use anyhow::Result;
use core::fmt::Write;

use super::{ansi::Terminal, serial::SerialConsole};
/// A ANSI compatible terminal.
pub struct SerialTerminal<T: SerialConsole, U: Terminal> {
    backend: T,
    console: U,
}

impl<T: SerialConsole, U: Terminal> SerialTerminal<T, U> {
    /// Create console from the given backend and terminal control configuration.
    pub fn from_serial_impl(b: T, c: U) -> Self {
        Self {
            backend: b,
            console: c,
        }
    }

    /// Destruct console, and give backend back to caller.
    /// This function does not reset terminal.
    pub fn release(self) -> T {
        self.backend
    }

    pub fn reset(&mut self) -> Result<()> {
        self.backend.write(U::RESET.as_bytes())?;
        Ok(())
    }
}

impl<T: SerialConsole, U: Terminal> Write for SerialTerminal<T, U> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        todo!()
    }
}

impl<T: SerialConsole, U: Terminal> Read<Key> for SerialTerminal<T, U> {
    fn read(&mut self, buf: &mut [Key]) -> Result<i32> {
        todo!()
    }
}

impl<T: SerialConsole, U: Terminal> ReadOne<Key> for SerialTerminal<T, U> {
    fn read_one(&mut self) -> Result<Key> {
        todo!()
    }
}

impl<T: SerialConsole, U: Terminal> Console for SerialTerminal<T, U> {
    fn wait_for_key(&mut self) -> Result<Key> {
        todo!()
    }

    fn read_key(&mut self) -> Result<Option<Key>> {
        todo!()
    }

    fn set_cursor(&mut self) -> Result<()> {
        todo!()
    }

    fn write_with_style(&mut self, text: &[u8], style: ConsoleStyle) -> Result<()> {
        todo!()
    }

    fn get_cursor(&mut self) -> Result<()> {
        todo!()
    }
}
