use core::fmt::Write;

use super::meta::ReadOne;
use crate::console::console::ConsoleStyle;
use anyhow::{anyhow, Result};
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

///
#[derive(Debug, Clone)]
pub struct TerminalInfo {
    /// Terminal Size.
    pub size: (usize, usize),
}

#[derive(Debug, Clone)]
pub enum CursorStyle {
    /// Hide cursor.
    None,
    /// Display cursor.
    Display,
}

/// Text console
///
/// Simple console with cursor control.
pub trait Console: Write + ReadOne<Key> {
    /// Wait until a key pressed.
    fn wait_for_key(&mut self) -> Result<Key> {
        self.read_one()
    }

    /// Read a key input immediately.
    fn read_key(&mut self) -> Result<Option<Key>> {
        self.read_one().map(|x| Some(x))
    }

    /// Get current cursor position in terminal.
    ///
    /// The upper left corner of the screen is defined as coord (0, 0) .
    fn get_cursor(&mut self) -> Result<(i32, i32)>;

    /// Set cursor position in terminal.
    ///
    /// An error will be generated if cursor position is invalid.
    fn set_cursor(&mut self, x: i32, y: i32) -> Result<()>;

    /// Get terminal information.
    fn terminal_info(&mut self) -> Result<TerminalInfo> {
        todo!()
    }

    /// Set cursor style.
    ///
    /// **NOTE**: This method may only return error if underlying console reports problem.
    /// If given cursor style does not exist, console driver should set cursor style to the most appropriate one or simply do nothing.
    fn set_cursor_style(&mut self, style: &CursorStyle) -> Result<()> {
        Ok(())
    }

    /// Try resizing terminal, and return dimension if succeed.
    fn resize(&mut self) -> Result<(i32, i32)> {
        Err(anyhow!("this terminal does not support auto resizing"))
    }

    /// Write text at current curson with given style.
    fn write_with_style(&mut self, text: &str, _style: ConsoleStyle) -> Result<()> {
        self.write_str(text)
            .map_err(|_| anyhow!("failed to write!"))
    }

    /// Reset console to a known clean state.
    ///
    /// This includes clearing screen, reset cursor, text style and key binding if possible.
    fn reset(&mut self) -> Result<()>;
}
