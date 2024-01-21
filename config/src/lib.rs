#![cfg_attr(feature = "no_std", no_std)]

pub mod boot;
pub mod bootconf;

pub use boot::{BootItem, BootTarget, Config, Extent, ImageLocation};
pub use bootconf::{BootOption, BootOptionItem, BootOptionKind, BootOptionSelection};
