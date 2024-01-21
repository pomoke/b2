use core::fmt::Write;

use super::meta::ReadOne;
use crate::console::console::ConsoleStyle;
use anyhow::{anyhow, Result};
use uefi::proto::console::text::{Key as EFIKey, ScanCode};
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

#[derive(Debug, Clone)]
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
            EFIKey::Special(ScanCode::FUNCTION_1) => Self::Accelerator(AcceleratorKey::F(1)),
            EFIKey::Special(ScanCode::FUNCTION_2) => Self::Accelerator(AcceleratorKey::F(2)),
            EFIKey::Special(ScanCode::FUNCTION_3) => Self::Accelerator(AcceleratorKey::F(3)),
            EFIKey::Special(ScanCode::FUNCTION_4) => Self::Accelerator(AcceleratorKey::F(4)),
            EFIKey::Special(ScanCode::FUNCTION_5) => Self::Accelerator(AcceleratorKey::F(5)),
            EFIKey::Special(ScanCode::FUNCTION_6) => Self::Accelerator(AcceleratorKey::F(6)),
            EFIKey::Special(ScanCode::FUNCTION_7) => Self::Accelerator(AcceleratorKey::F(7)),
            EFIKey::Special(ScanCode::FUNCTION_8) => Self::Accelerator(AcceleratorKey::F(8)),
            EFIKey::Special(ScanCode::FUNCTION_9) => Self::Accelerator(AcceleratorKey::F(9)),
            EFIKey::Special(ScanCode::FUNCTION_10) => Self::Accelerator(AcceleratorKey::F(10)),
            EFIKey::Special(ScanCode::FUNCTION_11) => Self::Accelerator(AcceleratorKey::F(11)),
            EFIKey::Special(ScanCode::FUNCTION_12) => Self::Accelerator(AcceleratorKey::F(12)),
            EFIKey::Special(k) => Self::Unknown(k.0 as u8),
        }
    }
}

/// Terminal Methods
///
/// Text console with cursor control, reverse text and keyboard handler.
///
/// No cursor display control or echo control here.
/// Cursor is always on, and to get a line secretly, use ReadSecret trait.
pub trait Console: Write + ReadOne<Key> {
    /// Wait until a key pressed.
    fn wait_for_key(&mut self) -> Result<Key> {
        self.read_one()
    }
    fn read_key(&mut self) -> Result<Option<Key>> {
        self.read_one().map(|x| Some(x))
    }

    /// Get current cursor position in terminal.
    ///
    /// This function may always fail for some type of terminals.
    fn get_cursor(&mut self) -> Result<()> {
        Err(anyhow!("get_cursor() unsupported for current terminal."))
    }
    /// Set cursor position in terminal.
    ///
    /// An error will be generated if cursor position is invalid.
    fn set_cursor(&mut self) -> Result<()>;
    /// Write text at current curson with given style.
    fn write_with_style(&mut self, text: &str, _style: ConsoleStyle) -> Result<()> {
        self.write_str(text)
            .map_err(|_| anyhow!("failed to write!"))
    }

    fn reset(&mut self) -> Result<()> {
        todo!()
    }
}
