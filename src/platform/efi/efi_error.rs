use alloc::collections::TryReserveError;
use anyhow::{anyhow, Result};
use core::fmt::Debug;

/// Turns some non `core::err::Error` type to `core::err::Error` (nightly only), so can be used with crates like anyhow.
pub trait ToError<U> {
    fn core_err(self) -> Result<U>;
}

impl<T: Debug, U> ToError<U> for uefi::Result<U, T> {
    fn core_err(self) -> Result<U> {
        self.map_err(|x| anyhow!("UEFI error, status {}", x.status()))
    }
}

impl ToError<()> for core::fmt::Result {
    fn core_err(self) -> Result<()> {
        // fmt::Error does not include any reason.
        self.map_err(|_| anyhow!("failed to format."))
    }
}

impl<U> ToError<U> for Result<U, TryReserveError> {
    fn core_err(self) -> Result<U> {
        self.map_err(|x| anyhow!("failed to allocate, reason {}", x))
    }
}
