use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Build a config interactively.
    Wizard { output: PathBuf },
    /// Check if a config is **semantically** valid.
    Check { config: PathBuf },
    /// Convert TOML/yaml config to JSON.
    Convert {
        input: PathBuf,
        output: Option<PathBuf>,
    },
}
