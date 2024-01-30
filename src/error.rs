use anyhow::{anyhow, Result};
use core::fmt::Debug;

/// Turns some non `core::err::Error` type to `core::err::Error`, so can be used with something like anyhow.
pub trait ToError<U> {
    fn core_err(self) -> Result<U>;
}

impl<T: Debug, U> ToError<U> for uefi::Result<U, T> {
    fn core_err(self) -> Result<U> {
        self.map_err(|x| anyhow!("UEFI error, status {}", x.status()))
    }
}
