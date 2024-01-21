//! Serial-Based Console
//! The terminal requires a transport and a control implementation.
//! That is, you can use it for any stream based and ANSI compatable in-band controlled console.

use crate::console::console::ConsoleStyle;
use crate::io::ReadOne;
use crate::io::{
    console::{Console, Key},
    Read,
};
use anyhow::Result;
use core::fmt::Write;

use super::{ansi::ANSIConsole, serial::SerialConsole};
