use std::path::PathBuf;
use std::sync::Arc;

use dropsquash_core::{
    default_history_path, AppError, EncodeJob, LicenseState, OutputSize, Profile, SourcePolicy,
};
use dropsquash_encoder::EncoderBackend;
use dropsquash_fileguard::{wait_until_stable, StabilityOptions};
use dropsquash_history::{append_record, ConversionRecord};
use tokio_util::sync::CancellationToken;

use super::dto::ConversionSummary;
use super::format_error;
use super::license::current_license_state;
use super::progress::WindowProgressReporter;
use crate::state::AppState;

#[cfg(target_os = "linux")]
use dropsquash_encoder::GStreamerEncoder as NativeEncoder;
#[cfg(target_os = "windows")]
use dropsquash_encoder::MediaFoundationEncoder as NativeEncoder;
#[cfg(target_os = "macos")]
use dropsquash_encoder::VideoToolboxEncoder as NativeEncoder;

pub async fn convert(
    app_state: tauri::State<'_, AppState>,
    window: tauri::WebviewWindow,
    input_path: String,
    output_dir: String,
    profile: Profile,
    output_size: OutputSize,
) -> std::result::Result<ConversionSummary, String> {
    let cancel = app_state.start_conversion()?;
    let result = convert_inner(window, input_path, output_dir, profile, output_size, cancel).await;
    app_state.finish_conversion();
    result
}

async fn convert_inner(
    window: tauri::WebviewWindow,
    input_path: String,
    output_dir: String,
    profile: Profile,
    output_size: OutputSize,
    cancel: CancellationToken,
) -> std::result::Result<ConversionSummary, String> {
    ensure_trial_open().await?;
    let input_path = PathBuf::from(input_path);
    ensure_supported_input(&input_path)?;
    wait_until_stable(&input_path, StabilityOptions::default(), cancel.clone())
        .await
        .map_err(format_error)?;

    let result = NativeEncoder
        .encode_with_progress_and_cancel(
            EncodeJob {
                input_path,
                output_dir: PathBuf::from(output_dir),
                profile,
                output_size,
                source_policy: SourcePolicy::Ask,
            },
            Arc::new(WindowProgressReporter { window }),
            cancel,
        )
        .await
        .map_err(format_error)?;
    append_record(
        &default_history_path(),
        &ConversionRecord::new(result.clone()),
    )
    .await
    .map_err(format_error)?;
    Ok(ConversionSummary::from(result))
}

async fn ensure_trial_open() -> Result<(), String> {
    if matches!(
        current_license_state().await.map_err(format_error)?,
        LicenseState::Locked(_)
    ) {
        return Err("Trial complete. Enter a license key to continue.".to_string());
    }
    Ok(())
}

fn ensure_supported_input(input_path: &std::path::Path) -> Result<(), String> {
    let capabilities = NativeEncoder.probe_capabilities().map_err(format_error)?;
    let extension = input_path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase)
        .unwrap_or_default();
    if capabilities
        .input_extensions
        .iter()
        .any(|allowed| allowed == &extension)
    {
        return Ok(());
    }
    Err(format_error(AppError::UnsupportedMedia(extension)))
}
