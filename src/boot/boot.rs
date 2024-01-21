//! Traits for boot.

use alloc::{string::String, vec::Vec};
use anyhow::Result;

pub struct WithParam;
pub trait BootAble {
    /// Make a instance of boot target.
    fn load(&mut self) -> Result<()>;
    /// Boot this target.
    fn boot(&mut self) -> Result<!>;
}
