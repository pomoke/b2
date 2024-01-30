use crate::console::serial::SerialConsole;
use crate::io::console::Key;
use crate::io::{console::Console, Read};
use core::fmt::Write;
use uefi::{
    proto::console::text::{Input, Key as EFIKey, Output},
    table::boot::ScopedProtocol,
    Error,
};

/// Style of terminal text
#[derive(Debug, Clone)]
pub struct ConsoleStyle {
    /// Bold text.
    bold: bool,
    /// Highlighted text.
    reverse: bool,
    /// Blinking text.
    ///
    /// Note: This style may be disractive.
    blink: bool,
}

impl ConsoleStyle {
    const BOLD: Self = Self {
        bold: true,
        reverse: false,
        blink: false,
    };
    const REV: Self = Self {
        bold: false,
        reverse: true,
        blink: false,
    };
    const BOLDREV: Self = Self {
        bold: true,
        reverse: true,
        blink: false,
    };
}

/// Console Information
pub struct ConsoleInfo {
    width: i32,
    height: i32,
}
