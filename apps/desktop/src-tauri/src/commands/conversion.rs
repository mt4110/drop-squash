use std::path::PathBuf;
use std::sync::Arc;

use dropsquash_core::{default_history_path, EncodeJob, LicenseState};
use dropsquash_encoder::EncoderBackend;
use dropsquash_fileguard::{wait_until_stable, StabilityOptions};
use dropsquash_history::append_successful_record;
use dropsquash_privacy::PrivacyReceipt;
use tokio_util::sync::CancellationToken;

use super::dto::{ConversionSummary, ConvertRequest};
use super::format_error;
use super::license::current_license_state;
use super::progress::WindowProgressReporter;
use super::source::handle_source_action;
use crate::state::AppState;

mod input;

#[cfg(target_os = "macos")]
use dropsquash_encoder::AppleNativeEncoder as NativeEncoder;
#[cfg(target_os = "linux")]
use dropsquash_encoder::GStreamerEncoder as NativeEncoder;
#[cfg(target_os = "windows")]
use dropsquash_encoder::MediaFoundationEncoder as NativeEncoder;

pub async fn convert(
    app_state: tauri::State<'_, AppState>,
    window: tauri::WebviewWindow,
    request: ConvertRequest,
) -> std::result::Result<ConversionSummary, String> {
    ensure_trial_open().await?;
    let cancel = app_state.start_conversion()?;
    let result = convert_inner(window, request, cancel).await;
    app_state.finish_conversion();
    result
}

async fn convert_inner(
    window: tauri::WebviewWindow,
    request: ConvertRequest,
    cancel: CancellationToken,
) -> std::result::Result<ConversionSummary, String> {
    let input_path = PathBuf::from(request.input_path);
    input::ensure_supported(&input_path)?;
    wait_until_stable(&input_path, StabilityOptions::default(), cancel.clone())
        .await
        .map_err(format_error)?;

    let result = NativeEncoder
        .encode_with_progress_and_cancel(
            EncodeJob {
                input_path,
                output_dir: PathBuf::from(request.output_dir),
                profile: request.profile,
                output_size: request.output_size,
                source_policy: request.source_policy,
            },
            Arc::new(WindowProgressReporter { window }),
            cancel.clone(),
        )
        .await
        .map_err(format_error)?;
    ensure_not_cancelled(&cancel)?;
    let receipt_path = if request.write_privacy_receipt {
        Some(PrivacyReceipt::save_for_result(&result).map_err(format_error)?)
    } else {
        None
    };
    append_successful_record(&default_history_path(), result.clone())
        .await
        .map_err(format_error)?;
    let decision = handle_source_action(&result, request.source_policy)?;
    Ok(ConversionSummary::new(result, decision, receipt_path))
}

async fn ensure_trial_open() -> Result<(), String> {
    reject_locked_license(current_license_state().await.map_err(format_error)?)
}

fn reject_locked_license(state: LicenseState) -> Result<(), String> {
    match state {
        LicenseState::Locked(_) => {
            Err("Trial complete. Enter a license key to continue.".to_string())
        }
        LicenseState::Trial(_) | LicenseState::Pro => Ok(()),
    }
}

fn ensure_not_cancelled(cancel: &CancellationToken) -> Result<(), String> {
    if cancel.is_cancelled() {
        return Err("Conversion cancelled.".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests;
