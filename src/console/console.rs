use core::ops::BitOr;

/// Style of terminal text
#[derive(Debug, Clone)]
pub struct ConsoleStyle {
    /// Bold text.
    bold: bool,
    /// Highlighted text.
    reverse: bool,
    /// Blinking text.
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
