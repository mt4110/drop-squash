use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use dropsquash_core::{OutputSize, Profile};

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

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ProfileArg {
    Auto,
    Slack,
    Docs,
    Teams,
    Discord,
    Chatwork,
    Line,
    #[value(name = "whatsapp")]
    WhatsApp,
    Archive,
    Privacy,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputSizeArg {
    Auto,
    #[value(name = "1080p")]
    P1080,
    #[value(name = "720p")]
    P720,
    #[value(name = "480p")]
    P480,
}

impl From<ProfileArg> for Profile {
    fn from(value: ProfileArg) -> Self {
        match value {
            ProfileArg::Auto => Self::Auto,
            ProfileArg::Slack => Self::Slack,
            ProfileArg::Docs => Self::Docs,
            ProfileArg::Teams => Self::Teams,
            ProfileArg::Discord => Self::Discord,
            ProfileArg::Chatwork => Self::Chatwork,
            ProfileArg::Line => Self::Line,
            ProfileArg::WhatsApp => Self::WhatsApp,
            ProfileArg::Archive => Self::Archive,
            ProfileArg::Privacy => Self::Privacy,
        }
    }
}

impl From<OutputSizeArg> for OutputSize {
    fn from(value: OutputSizeArg) -> Self {
        match value {
            OutputSizeArg::Auto => Self::Auto,
            OutputSizeArg::P1080 => Self::P1080,
            OutputSizeArg::P720 => Self::P720,
            OutputSizeArg::P480 => Self::P480,
        }
    }
}
