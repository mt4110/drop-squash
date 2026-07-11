mod convert;
mod doctor;
mod license;
mod stats;

use crate::args::{Cli, Command};

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
compile_error!("DropSquash supports macOS, Windows, and Linux");

pub async fn run(cli: Cli) -> dropsquash_core::Result<()> {
    match cli.command {
        Command::Convert {
            input,
            output_dir,
            profile,
            output_size,
            history,
        } => convert::run(input, output_dir, profile, output_size, history).await,
        Command::Stats { history } => stats::run(history).await,
        Command::Doctor => doctor::run(),
    }
}
