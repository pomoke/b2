//! ANSI console support.
extern crate alloc;
use super::console::ConsoleStyle;
use alloc::string::String;

/// trait for control sequence-based terminal
///
/// If a terminal does not support specified feature, implement a no-op.
pub trait Terminal {
    /// Apply style to s for terminal.
    fn apply_style(s: &str, style: ConsoleStyle) -> String;

    /// Set active cursor position.
    fn set_pos(x: i32, y: i32) -> String;
    /// Terminal reset command.
    const RESET: &'static str;
    /// Enable echo.
    const ECHO_ON: &'static str;
    /// Disable echo.
    const ECHO_OFF: &'static str;
}
/// ANSI compatible terminal
pub struct ANSIConsole {}

impl Terminal for ANSIConsole {
    fn apply_style(s: &str, style: ConsoleStyle) -> String {
        String::new()
    }
    fn set_pos(x: i32, y: i32) -> String {
        String::new()
    }

    const RESET: &'static str = "\x1b[";
    const ECHO_ON: &'static str = "\x1b[";
    const ECHO_OFF: &'static str = "\x1b[";
}
