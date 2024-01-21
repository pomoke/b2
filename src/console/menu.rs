use anyhow::Result;

use crate::io::console::Console;
use config::boot::{BootItem, Config};

/// Show menu, then retrieve user selection.
///
/// This is a very high-level trait.
pub trait Menu {
    fn prompt(&self, config: &Config, console: &mut dyn Console) -> Result<&BootItem>;
}
