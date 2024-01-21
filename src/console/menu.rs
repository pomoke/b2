use anyhow::Result;

use crate::{
    config::{BootItem, Config},
    io::console::Console,
};

/// Show menu, then retrieve user selection.
///
/// This is a very high-level trait.
pub trait Menu {
    fn prompt(&self, config: &Config, console: &mut dyn Console) -> Result<&BootItem>;
}
