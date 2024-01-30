use core::ops::{Deref, DerefMut};

use once_cell::unsync::OnceCell;

/// A simple wrapper around OnceCell.
///
/// If failed to acquire reference, the program will panic.
pub struct InitedCell<T>(OnceCell<T>);

impl<T> InitedCell<T> {
    pub fn get(&self) -> &T {
        self.0.get().unwrap()
    }
}

impl<T> Deref for InitedCell<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.get()
    }
}
