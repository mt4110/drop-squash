use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod values;

pub use values::{OutputSizeArg, ProfileArg};

#[derive(Debug, Parser)]
#[command(name = "dropsquash")]
#[command(about = "Drop huge screen recordings. Squash them locally.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Convert {
        input: PathBuf,
        #[arg(long)]
        output_dir: PathBuf,
        #[arg(long, value_enum, default_value_t = ProfileArg::Auto)]
        profile: ProfileArg,
        #[arg(long, value_enum, default_value_t = OutputSizeArg::Auto)]
        output_size: OutputSizeArg,
        #[arg(long)]
        history: Option<PathBuf>,
    },
    Stats {
        #[arg(long)]
        history: Option<PathBuf>,
    },
    License {
        #[command(subcommand)]
        command: LicenseCommand,
    },
    Receipt {
        output: PathBuf,
    },
    Doctor,
}

#[derive(Debug, Subcommand)]
pub enum LicenseCommand {
    Status {
        #[arg(long)]
        history: Option<PathBuf>,
    },
    Forget,
}
