//! ANSI console support.
//!
//! This may be useful for serial consoles.
extern crate alloc;
use super::console::ConsoleStyle;
use alloc::string::String;
use anyhow::Result;

/// trait for control sequence-based terminal
///
/// If a terminal does not support specified feature, implement a no-op.

pub trait Terminal {}

/// ANSI compatible terminal.
///
/// Most in-band control terminals is ANSI compatible, so there is no much need to abstract this aspect.
pub struct ANSIConsole {}

#[allow(unused)]
impl ANSIConsole {
    pub const ESCAPE: &'static str = "\x1b[";
    pub const RESET: &'static str = "\x1bc";

    pub const BLACK: i32 = 30;
    pub const RED: i32 = 31;
    pub const GREEN: i32 = 32;
    pub const YELLOW: i32 = 33;
    pub const BLUE: i32 = 34;
    pub const MAGANTA: i32 = 35;
    pub const CYAN: i32 = 36;
    pub const WHITE: i32 = 37;

    pub const BACKGROUND_OFFSET: i32 = 10;

    pub fn apply_style(s: &str, style: ConsoleStyle) -> String {
        String::new()
    }

    pub fn set_pos(x: i32, y: i32) -> String {
        String::new()
    }

    pub fn reset() -> Result<()> {
        Ok(())
    }
}
