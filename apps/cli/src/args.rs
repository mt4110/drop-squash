use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod secure_share;
mod values;

pub use secure_share::{MaskModeArg, MaskRectArg};
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
        #[arg(long, value_enum)]
        secure_share_mode: Option<MaskModeArg>,
        #[arg(long = "secure-share-rect")]
        secure_share_rects: Vec<MaskRectArg>,
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
    VerifyEvidence {
        video: PathBuf,
        sidecar: PathBuf,
    },
    VerifySecureShareOutput {
        video: PathBuf,
        sidecar: PathBuf,
    },
    Doctor,
}

#[derive(Debug, Subcommand)]
pub enum LicenseCommand {
    Status {
        #[arg(long)]
        history: Option<PathBuf>,
        #[arg(long)]
        cache_path: Option<PathBuf>,
    },
    Forget {
        #[arg(long)]
        cache_path: Option<PathBuf>,
    },
}
