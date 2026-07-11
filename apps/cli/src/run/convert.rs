use std::path::PathBuf;

use dropsquash_core::{default_history_path, AppError, EncodeJob, LicenseState, SourcePolicy};
use dropsquash_encoder::EncoderBackend;
use dropsquash_history::{append_successful_record, read_records, HistoryMetrics};

use crate::args::{OutputSizeArg, ProfileArg};

use super::license;

#[cfg(target_os = "linux")]
use dropsquash_encoder::GStreamerEncoder as NativeEncoder;
#[cfg(target_os = "windows")]
use dropsquash_encoder::MediaFoundationEncoder as NativeEncoder;
#[cfg(target_os = "macos")]
use dropsquash_encoder::VideoToolboxEncoder as NativeEncoder;

pub async fn run(
    input: PathBuf,
    output_dir: PathBuf,
    profile: ProfileArg,
    output_size: OutputSizeArg,
    history: Option<PathBuf>,
) -> dropsquash_core::Result<()> {
    let history = history.unwrap_or_else(default_history_path);
    ensure_trial_available(&history).await?;
    let result = NativeEncoder
        .encode(EncodeJob {
            input_path: input,
            output_dir,
            profile: profile.into(),
            output_size: output_size.into(),
            source_policy: SourcePolicy::Ask,
        })
        .await?;
    append_successful_record(&history, result.clone()).await?;
    println!("output: {}", result.output_path.display());
    println!("original bytes: {}", result.original_bytes);
    println!("squashed bytes: {}", result.output_bytes);
    println!("saved bytes: {}", result.saved_bytes());
    println!("reduction: {:.2}%", result.reduction_percent());
    license::print_state(license::state(&history).await?);
    Ok(())
}

async fn ensure_trial_available(history: &std::path::Path) -> dropsquash_core::Result<()> {
    let records = read_records(history).await?;
    let metrics = HistoryMetrics::from_records(&records);
    if matches!(
        license::gate()?.state_for_metrics(metrics),
        LicenseState::Locked(_)
    ) {
        return Err(AppError::License(
            "trial limit reached; enter a license key to continue".to_string(),
        ));
    }
    Ok(())
}
