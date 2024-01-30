extern crate alloc;
use alloc::borrow::Cow;

pub struct Area {
    height: i32,
    width: i32,
}
pub trait Component {}

pub struct Frame {}

pub struct Text<'a> {
    pub text: Cow<'a, &'a str>,
    pub style: TextStyle,
}

impl<'a> Component for Text<'a> {}

pub struct TextStyle {
    pub bold: bool,
    pub underline: bool,
    pub size: Option<u8>,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            bold: false,
            underline: false,
            size: None,
        }
    }
}

impl TextStyle {
    const Bold: Self = TextStyle {
        bold: true,
        underline: false,
        size: None,
    };
    const Underline: Self = TextStyle {
        underline: true,
        bold: false,
        size: None,
    };
}

pub struct Image<'a> {
    buf: &'a u8,
    size: Area,
    format: Color,
}

impl<'a> Component for Image<'a> {}

pub enum Color {
    /// Transparent if possible, or black/white.
    None,
    Black,
    White,
    Grayscale(u8),
    RGB(u8, u8, u8),
}

impl Color {
    pub const Green: Self = Color::RGB(0, 255, 0);
}
