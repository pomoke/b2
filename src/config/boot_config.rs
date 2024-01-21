//! Boot time option.
//!
//! This can be used to compose boot options with an user-friendly interface.

use alloc::{string::String, vec, vec::Vec};
use anyhow::{anyhow, Result};
use config::bootconf::{BootOption, BootOptionItem, BootOptionKind, BootOptionSelection};
use serde::{Deserialize, Serialize};

extern crate alloc;

pub fn make_parameter(
    options: &[BootOption],
    selections: &[BootOptionSelection],
) -> Result<Vec<String>> {
    if options.len() != selections.len() {
        return Err(anyhow!("option and selection length mismatch."));
    }
    let cmdline_len = options.iter().map(|x| x.pos).max().unwrap() as usize + 1;
    let mut ret = vec![String::new(); cmdline_len];

    for (i, j) in options.iter().zip(selections) {
        match (&i.option, j) {
            (BootOptionKind::Bool(cmdline), BootOptionSelection::Bool(selected)) => {
                if *selected {
                    let target = &mut ret[i.pos as usize];
                    target.push_str(cmdline);
                    target.push(' ');
                }
            }
            (BootOptionKind::Multiple(variants), BootOptionSelection::Multiple(which)) => {
                let target = &mut ret[i.pos as usize];
                let variant = variants
                    .get(*which as usize)
                    .ok_or_else(|| anyhow!("index out of range"))?;
                target.push_str(variant.value.as_str());
                target.push(' ');
            }
            (BootOptionKind::Template(template), BootOptionSelection::Template(value)) => {
                if let Some(value) = value {
                    let target = &mut ret[i.pos as usize];
                    target.push_str(template);
                    target.push_str(value);
                    target.push(' ');
                }
            }
            _ => return Err(anyhow!("option type and selection type mismatch.")),
        }
    }

    Ok(ret)
}
