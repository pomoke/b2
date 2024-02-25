use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Check if the config is **semantically** valid.
    Check { config: PathBuf },
    /// Send sample config file to stdout.
    Sample,
    /// Generate hashed password for use with b2.
    Password {
        /// Memory size for argon2id, in KB.
        #[arg(short)]
        m: Option<u32>,
        /// Number of iterations for argon2id.
        #[arg(short)]
        t: Option<u32>,
    },
}
