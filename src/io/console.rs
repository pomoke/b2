use core::fmt::Write;

use super::{meta::ReadOne, Read};
use crate::console::console::ConsoleStyle;
use anyhow::{anyhow, Result};
use uefi::{
    proto::console::text::{Key as EFIKey, ScanCode},
    Char16,
};
/// Key struct - represents a key pressed.
#[derive(Debug, Clone)]
pub enum Key {
    /// Printable characters
    Printable(char),
    /// Non-printable key press, including arrow keys.
    Accelerator(AcceleratorKey),
    /// Unknown key press received.
    Unknown(u8),
}

#[derive(Debug, Clone)]
pub enum AcceleratorKey {
    Esc,
    Tab,
    PrtSc,
    PgUp,
    PgDn,
    Up,
    Down,
    Left,
    Right,
    Backspace,
    Home,
    End,
    Insert,
    Delete,
    Enter,
    F(u8),
}

pub enum ModifierKey {
    Ctrl,
    Shift,
    Alt,
}

impl Key {}

#[cfg(target_os = "uefi")]
impl From<EFIKey> for Key {
    fn from(value: EFIKey) -> Self {
        match value {
            EFIKey::Printable(u) => {
                let u: char = u.into();
                match u {
                    '\x08' => Self::Accelerator(AcceleratorKey::Backspace),
                    '\t' => Self::Accelerator(AcceleratorKey::Tab),
                    '\r' => Self::Accelerator(AcceleratorKey::Enter),
                    k => Self::Printable(k),
                }
            }
            EFIKey::Special(ScanCode::UP) => Self::Accelerator(AcceleratorKey::Up),
            EFIKey::Special(ScanCode::DOWN) => Self::Accelerator(AcceleratorKey::Down),
            EFIKey::Special(ScanCode::LEFT) => Self::Accelerator(AcceleratorKey::Left),
            EFIKey::Special(ScanCode::RIGHT) => Self::Accelerator(AcceleratorKey::Right),
            EFIKey::Special(ScanCode::PAGE_UP) => Self::Accelerator(AcceleratorKey::PgUp),
            EFIKey::Special(ScanCode::PAGE_DOWN) => Self::Accelerator(AcceleratorKey::PgDn),
            EFIKey::Special(ScanCode::ESCAPE) => Self::Accelerator(AcceleratorKey::Esc),
            EFIKey::Special(ScanCode::HOME) => Self::Accelerator(AcceleratorKey::Home),
            EFIKey::Special(ScanCode::END) => Self::Accelerator(AcceleratorKey::End),
            EFIKey::Special(ScanCode::INSERT) => Self::Accelerator(AcceleratorKey::Insert),
            EFIKey::Special(ScanCode::DELETE) => Self::Accelerator(AcceleratorKey::Delete),
            EFIKey::Special(k) => Self::Unknown(k.0 as u8),
        }
    }
}

/// Terminal Struct
///
/// Text console with cursor control, reverse text and keyboard handler.
pub trait Console: Write + ReadOne<Key> {
    /// Wait until a key pressed.
    fn wait_for_key(&mut self) -> Result<Key>;
    fn read_key(&mut self) -> Result<Option<Key>>;
    /// Get current cursor position in terminal.
    ///
    /// If a terminal does not support cursor position query,
    /// then the terminal driver is required to maintain the state.
    fn get_cursor(&mut self) -> Result<()>;
    /// Set cursor position in terminal.
    ///
    /// An error will be generated if cursor position is invalid.
    fn set_cursor(&mut self) -> Result<()>;
    /// Write given text with given style.
    fn write_with_style(&mut self, text: &[u8], style: ConsoleStyle) -> Result<()> {
        todo!()
    }

    fn reset(&mut self) -> Result<()> {
        todo!()
    }
}
