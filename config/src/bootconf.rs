//! Boot time option.
//!
//! This can be used to compose boot options with an user-friendly interface.

#[cfg(feature = "no_std")]
extern crate alloc;
#[cfg(feature = "no_std")]
use alloc::{string::String, vec, vec::Vec};
use serde::{Deserialize, Serialize};

/// User-selectable boot option.
///
/// No dependency check here.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BootOption {
    /// Short option identifier.
    ///
    /// This is used for searching and i18n.
    pub identifier: String,
    /// The cmdline to append to.
    ///
    /// Useful with multiboot where every module have its own cmdline.
    pub pos: i32,
    /// Human readable name
    pub name: String,
    /// Detailed description if available.
    pub description: Option<String>,
    pub option: BootOptionKind,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BootOptionKind {
    /// Yes/No option.
    ///
    /// Append to cmdline if selected.
    Bool(String),
    /// Multiple selection.
    ///
    /// Option name - append cmdline
    Multiple(Vec<BootOptionItem>),
    /// A template to be filled with user provided value.
    ///
    /// Example: `root={}`
    Template(String),
    /// Unknown, for compability
    #[serde(other)]
    Default,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BootOptionItem {
    pub identifier: String,
    pub name: String,
    pub description: Option<String>,
    pub value: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BootOptionSelection {
    Bool(bool),
    Multiple(i32),
    Template(Option<String>),
    #[serde(other)]
    Default,
}
