use crate::config::boot_config::BootOption;
use crate::config::boot_config::BootOptionKind;
use crate::config::boot_config::BootOptionSelection;
use crate::platform::ToError;
use alloc::format;
use alloc::string::String;
use anyhow::anyhow;
use anyhow::Result;

use crate::config::BootItem;
use crate::io::{LineEdit, ReadString};
use crate::{config::Config, io::console::Console};

use alloc::{vec, vec::Vec};

/// Basic Menu - Text based menu.
///
pub struct BasicMenu {}

impl BasicMenu {
    /// Show menu on basic console.
    ///
    pub fn render(config: &Config, console: &mut dyn Console) -> Result<()> {
        // Show title
        let title = config
            .name
            .as_ref()
            .map(|x| x.as_str())
            .unwrap_or("b2 loader");
        write!(console, "{}\n", title).map_err(|_| anyhow!("failed to write"))?;
        // Show message
        if let Some(msg) = config.message.as_ref() {
            write!(console, "{}\n", msg).map_err(|_| anyhow!("failed to write"))?;
        }
        // Show items.
        for (i, j) in config.items.iter().enumerate() {
            write!(console, "[{}]: {}\n", i + 1, j.name).map_err(|_| anyhow!("failed to write"))?;
        }
        // Show prompt.

        Ok(())
    }

    /// Read selected boot option.
    ///
    pub fn prompt<'a>(
        &self,
        config: &'a Config,
        console: &mut dyn Console,
    ) -> Result<&'a BootItem> {
        Self::render(config, console)?;
        let len = config.items.len();
        loop {
            let mut buf = String::new();
            console.edit_line(&mut buf, "Boot:")?;
            let buf = buf.trim();
            if buf.len() == 0 {
                continue;
            }
            let selection: Result<usize, _> = buf.parse();
            if let Ok(k) = selection {
                if k < 1 || k > len {
                    write!(console, "Invalid option.\n")
                        .map_err(|e| anyhow!("failed to write due to {}", e))?;
                    continue;
                }
                return Ok(&config.items[k - 1]);
            } else {
                write!(console, "Invalid option.\n")
                    .map_err(|e| anyhow!("failed to write due to {}", e))?;
                continue;
            }
        }
    }

    /// Read boot config by user.
    pub fn boot_config<'a>(
        &self,
        config: &'a [BootOption],
        console: &mut dyn Console,
    ) -> Result<()> {
        let mut buf = String::new();
        let mut ret: Vec<BootOptionSelection> = vec![];
        for i in config {
            loop {
                match &i.option {
                    BootOptionKind::Bool(k) => {
                        let prompt = format!("{}? (y/N/?) ", i.name);
                        console.edit_line(&mut buf, &prompt)?;
                        match buf.as_str() {
                            "y" | "Y" => {
                                ret.push(BootOptionSelection::Bool(true));
                                break;
                            }
                            "n" | "N" | "" => {
                                ret.push(BootOptionSelection::Bool(false));
                                break;
                            }
                            "?" => {
                                write!(
                                    console,
                                    "Option \"{}\" ({}): {}\n",
                                    i.name,
                                    i.identifier,
                                    i.description
                                        .as_deref()
                                        .unwrap_or("(description unavailable)")
                                )
                                .core_err()?;
                            }
                            _ => {
                                write!(console, "{} is not a valid option.\n", buf).core_err()?;
                            }
                        }
                    }
                    BootOptionKind::Multiple(k) => {
                        write!(console, "{}:\n", i.name).core_err()?;
                        for (i, item) in k.iter().enumerate() {
                            // Indent is intentional.
                            write!(
                                console,
                                "{} {}. {}\n",
                                if i == 0 { "*" } else { " " },
                                i + 1,
                                item.name
                            )
                            .core_err()?;
                        }

                        write!(console, "Select: ").core_err()?;
                        console.read_line(&mut buf)?;
                        if let Ok(num) = buf.parse::<i32>() {
                            if num < 0 || num > k.len() as i32 {
                                write!(console, "Invalid option.\n").core_err()?;
                                continue;
                            }
                            ret.push(BootOptionSelection::Multiple(num - 1));
                            break;
                        } else {
                            write!(console, "Invalid option.\n").core_err()?;
                            continue;
                        }
                    }
                    BootOptionKind::Template(k) => {
                        write!(
                            console,
                            "{} ({})= ",
                            i.name,
                            i.description.as_deref().unwrap_or("no description")
                        )
                        .core_err()?;
                        console.read_line(&mut buf)?;
                        ret.push(BootOptionSelection::Template(Some(buf.clone())));
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}
