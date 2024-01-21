use core::cmp::{max, min};

use alloc::string::String;
use alloc::{format, vec};
use anyhow::anyhow;
use anyhow::{Context, Result};
use log::debug;

use crate::platform::ToError;

use super::console::{AcceleratorKey, Console, CursorStyle, Key};
pub trait Handle: Sized {
    /// Close current resource.
    /// It's preferred to release resource in `trait drop`.
    fn close(self) {
        drop(self)
    }
}

pub trait Stream {
    fn read(&mut self, buf: &mut [u8]) -> Result<i32>;
    fn write(&mut self, buf: &[u8]) -> Result<i32>;
}

pub trait Read<T> {
    fn read(&mut self, buf: &mut [T]) -> Result<i32>;
}

/// Read one item of T.
pub trait ReadOne<T> {
    fn read_one(&mut self) -> Result<T>;
}

/// Read a line from terminal.
///
/// Does not provide line editing capability.
/// Note: This trait is not for reading password.
pub trait ReadString {
    /// Read a line, with echo.
    ///
    /// Do not use this function for secrets!
    /// Types of `Read<Key>` will automatically implement this trait.
    fn read_line(&mut self, buf: &mut String) -> Result<i32>;
}

/// Read a line of secret (such as password) from terminal.
///
/// Input should be hidden or masked.
pub trait ReadSecret {
    /// Read a line, secretly and with no echo at all.
    ///
    /// For those who don't use *nix, this may cause confusion.
    fn read_pass(&mut self, buf: &mut String) -> Result<i32>;

    /// Same as read_pass(), which input is masked with `*`
    ///
    /// Common practice in most UI, but not commonly used in *nix.
    fn read_pass_star(&mut self, buf: &mut String) -> Result<i32>;
}

impl<T: ?Sized + ReadOne<Key> + core::fmt::Write> ReadString for T {
    fn read_line(&mut self, buf: &mut String) -> Result<i32> {
        buf.clear();
        let mut n = 0;
        loop {
            let i = self.read_one()?;
            match i {
                Key::Printable(k) => {
                    n += 1;
                    buf.push(k);
                    self.write_char(k)
                        .map_err(|_| anyhow!("Failed to write to console."))?;
                }
                Key::Accelerator(AcceleratorKey::Enter) => {
                    self.write_str(if cfg!(target_os = "uefi") {
                        "\r\n"
                    } else {
                        "\n"
                    })
                    .map_err(|_| anyhow!("Failed to write to console."))?;
                    return Ok(n);
                }
                _ => {}
            }
        }
    }
}

pub trait Write<T> {
    fn write(&mut self, buf: &[T]) -> Result<i32>;
}

pub trait WriteString {
    fn write_str(&mut self, buf: &str) -> Result<i32>;
}

/// Block device meta-interface.
///
/// Note: this trait only support offset within 2^31-1 (2GB). This should be enough for bootloaders.
pub trait BlockDevice: Read<u8> {
    fn get_pos(&self) -> Result<i32>;
    fn set_pos(&mut self, pos: i32) -> Result<i32>;
}

pub struct EditConfig {}

/// Console with line edit capability.
pub trait LineEdit {
    /// Read a cleartext line.
    /// The line is editable, and may have default value.
    ///
    /// Returns string length.
    fn edit_line(&mut self, buf: &mut String, prompt: &str) -> Result<i32>;

    /// Edit an area of text.
    ///
    /// If this function is not implemented, `edit_line` will be called.
    fn edit_area(&mut self, buf: &mut String, config: &EditConfig) -> Result<i32> {
        Err(anyhow!("Not supported."))
    }
}

const LINEEDIT_MIN_LENGTH: usize = 32;

impl<T: ?Sized + Console> LineEdit for T {
    fn edit_line(&mut self, buf: &mut String, prompt: &str) -> Result<i32> {
        let size = self.terminal_info()?.size;
        //debug!("console {}x{}", size.0, size.1);
        let mut len = buf.len();
        // Put prompt and buf first.
        self.set_cursor_style(&CursorStyle::Display)?;
        self.write_str(prompt)
            .map_err(|_| anyhow!("failed to write"))?;
        self.write_str(buf)
            .map_err(|_| anyhow!("failed to write"))?;
        let mut cur_pos = self.get_cursor()?;
        // Cursor is at the end of buf.
        let mut str_pos = buf.len();
        let mut is_insert = true;
        let mut left_pos: usize = 0;

        let line = cur_pos.1;
        let available_width = size.0 - 1 - prompt.len();

        if size.0 < LINEEDIT_MIN_LENGTH {
            return Err(anyhow!(
                "console too small, expected at least {} cols, got {}.",
                LINEEDIT_MIN_LENGTH,
                size.0
            ));
        }

        // Read key input.
        loop {
            let key = self.wait_for_key()?;
            let mut redraw = false;
            cur_pos = self.get_cursor()?;
            match key {
                // Insert at current position.
                Key::Printable(k) => {
                    // Insert or replace character at current position.
                    if is_insert {
                        buf.insert(str_pos, k);
                        if str_pos + 1 == buf.len() {
                            // Append
                            redraw = (str_pos - left_pos + prompt.len() as usize) >= (size.0 - 2);
                            left_pos =
                                if (str_pos - left_pos + prompt.len() as usize) < (size.0 - 2) {
                                    left_pos
                                } else {
                                    left_pos + 1
                                };
                            if !redraw {
                                self.write_char(k).core_err()?
                            };
                        } else {
                            // Insert
                            redraw = true;
                            left_pos =
                                if (str_pos - left_pos + prompt.len() as usize) < (size.0 - 2) {
                                    left_pos
                                } else {
                                    left_pos + 1
                                };
                        }
                        str_pos += 1;
                        // redraw or print character out.
                    } else {
                        if str_pos == buf.len() {
                            // Append
                            buf.insert(str_pos, k);
                        } else {
                            // Replace
                            buf.replace_range(str_pos..(str_pos + 1), format!("{}", k).as_str());
                        }

                        // Always redraw on replace.
                        redraw = true;
                        left_pos = if (str_pos - left_pos + prompt.len() as usize) < (size.0 - 2) {
                            left_pos
                        } else {
                            left_pos + 1
                        };
                        str_pos += 1;
                    }
                }
                Key::Accelerator(AcceleratorKey::Enter) => {
                    self.write_char('\n')
                        .map_err(|_| anyhow!("failed to write"))?;
                    break;
                }
                // Delete before or after.
                Key::Accelerator(k @ (AcceleratorKey::Backspace | AcceleratorKey::Delete)) => {
                    match k {
                        AcceleratorKey::Backspace => {
                            if str_pos as isize - 1 >= 0 {
                                buf.replace_range((str_pos - 1)..str_pos, "");
                                redraw = true;
                                str_pos -= 1;
                            }
                        }
                        AcceleratorKey::Delete => {
                            if str_pos + 1 <= buf.len() {
                                buf.replace_range(str_pos..(str_pos + 1), "");
                                redraw = true;
                            }
                        }
                        _ => unreachable!(),
                    };
                }

                // Cursor/Viewport move.
                Key::Accelerator(
                    k @ (AcceleratorKey::Left
                    | AcceleratorKey::Right
                    | AcceleratorKey::Up
                    | AcceleratorKey::Down
                    | AcceleratorKey::PgUp
                    | AcceleratorKey::PgDn
                    | AcceleratorKey::Home
                    | AcceleratorKey::End),
                ) => match k {
                    AcceleratorKey::Left => {
                        if str_pos as isize - 1 >= 0 {
                            str_pos -= 1;
                            if left_pos >= str_pos {
                                left_pos = str_pos;
                            }
                            redraw = true;
                        }
                    }
                    AcceleratorKey::Right => {
                        if str_pos + 1 <= buf.len() {
                            str_pos += 1;
                            if str_pos - left_pos >= available_width {
                                left_pos += 1;
                            }
                            redraw = true;
                        }
                    }
                    AcceleratorKey::Home => {
                        str_pos = 0;
                        left_pos = 0;
                        redraw = true;
                    }
                    AcceleratorKey::End => {
                        str_pos = buf.len();
                        left_pos =
                            max((str_pos as isize) - (available_width as isize), 0isize) as usize;
                        redraw = true;
                    }
                    // Go one line up/front.
                    AcceleratorKey::Up | AcceleratorKey::PgUp => {
                        redraw = true;
                        if str_pos >= 2 * available_width {
                            str_pos -= available_width;
                            left_pos =
                                max(left_pos as isize - available_width as isize, 0) as usize;
                        } else {
                            str_pos = 0;
                            left_pos = max((str_pos as isize) - (available_width as isize), 0isize)
                                as usize;
                        }
                    }
                    // Go one line down/back.
                    AcceleratorKey::Down | AcceleratorKey::PgDn => {
                        redraw = true;
                        if str_pos + 2 * available_width <= buf.len() {
                            str_pos += available_width;
                            left_pos += available_width;
                        } else {
                            str_pos = buf.len();
                            left_pos = max((str_pos as isize) - (available_width as isize), 0isize)
                                as usize;
                        }
                    }
                    _ => unreachable!(),
                },

                // Switching between insert and replace mode.
                Key::Accelerator(AcceleratorKey::Insert) => {
                    is_insert = !is_insert;
                    // Change cursor shape, if possible.
                }

                // Spurious key input.
                _ => {}
            }
            // Erase display area and redisplay.
            if redraw {
                if str_pos - left_pos > available_width - 1 {
                    left_pos = str_pos - available_width + 1;
                }
                self.set_cursor(0, line)?;
                self.write_str(" ".repeat(size.0).as_str()).core_err()?;
                self.set_cursor(0, line)?;
                self.write_str(prompt).core_err()?;
                let display_buf = buf
                    .chars()
                    .skip(left_pos)
                    .take(available_width + 1)
                    .collect::<String>();
                self.write_str(&display_buf).core_err().context("write")?;
                self.set_cursor((prompt.len() + str_pos - left_pos) as i32, line)
                    .with_context(|| {
                        format!(
                            "attempt to locate cursor to {}, {}\n",
                            prompt.len() + str_pos - left_pos,
                            line
                        )
                    })?;
            }
        }
        self.set_cursor_style(&CursorStyle::None)?;
        Ok(0)
    }
}
