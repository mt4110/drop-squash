mod convert;
mod doctor;
mod evidence;
mod license;
mod receipt;
mod secure_share;
mod secure_share_output;
mod stats;

use crate::args::{Cli, Command, LicenseCommand};

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
            secure_share_mode,
            secure_share_rects,
        } => {
            convert::run(
                input,
                output_dir,
                profile,
                output_size,
                history,
                secure_share::options(secure_share_mode, secure_share_rects)?,
            )
            .await
        }
        Command::Stats { history } => stats::run(history).await,
        Command::License {
            command:
                LicenseCommand::Status {
                    history,
                    cache_path,
                },
        } => license::status(history, cache_path).await,
        Command::License {
            command: LicenseCommand::Forget { cache_path },
        } => license::forget(cache_path),
        Command::Receipt { output } => receipt::run(output),
        Command::VerifyEvidence { video, sidecar } => evidence::run(video, sidecar),
        Command::VerifySecureShareOutput { video, sidecar } => {
            secure_share_output::run(video, sidecar)
        }
        Command::Doctor => doctor::run(),
    }
}
