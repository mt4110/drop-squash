use std::path::PathBuf;
use std::sync::Arc;

use dropsquash_core::{default_history_path, EncodeJob, NOT_SMALLER_MESSAGE};
use dropsquash_encoder::{resolve_secure_share_options, EncoderBackend};
use dropsquash_fileguard::{wait_until_stable, StabilityOptions};
use dropsquash_history::append_successful_record;
use tokio_util::sync::CancellationToken;

use super::dto::{ConversionOutcome, ConversionSummary, ConvertRequest};
use super::{format_error, progress::WindowProgressReporter, source::handle_source_action};
use crate::state::AppState;
mod gate;
mod input;
mod receipt;
use gate::ensure_trial_open;

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
) -> std::result::Result<ConversionOutcome, String> {
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
) -> std::result::Result<ConversionOutcome, String> {
    let input_path = PathBuf::from(request.input_path);
    let secure_share = resolve_secure_share_options(&input_path, request.secure_share.as_ref())
        .map_err(format_error)?;
    input::ensure_supported(&input_path)?;
    wait_until_stable(&input_path, StabilityOptions::default(), cancel.clone())
        .await
        .map_err(format_error)?;

    let result = match NativeEncoder
        .encode_with_progress_and_cancel(
            EncodeJob {
                input_path,
                output_dir: PathBuf::from(request.output_dir),
                profile: request.profile,
                output_size: request.output_size,
                source_policy: request.source_policy,
                secure_share: secure_share.clone(),
            },
            Arc::new(WindowProgressReporter { window }),
            cancel.clone(),
        )
        .await
    {
        Ok(result) => result,
        Err(error) if is_not_smaller_error(&error) => {
            return Ok(ConversionOutcome::KeptOriginal {
                message: error.user_message(),
            });
        }
        Err(error) => return Err(format_error(error)),
    };
    ensure_not_cancelled(&cancel)?;
    let receipt = receipt::save_receipt(
        &result,
        secure_share.as_ref(),
        None,
        request.write_privacy_receipt,
    )
    .map_err(format_error)?;
    append_successful_record(&default_history_path(), result.clone())
        .await
        .map_err(format_error)?;
    let decision = handle_source_action(&result, request.source_policy)?;
    Ok(ConversionOutcome::Converted(ConversionSummary::new(
        result, decision, receipt,
    )))
}

fn ensure_not_cancelled(cancel: &CancellationToken) -> Result<(), String> {
    if cancel.is_cancelled() {
        return Err("Conversion cancelled.".to_string());
    }
    Ok(())
}

fn is_not_smaller_error(error: &dropsquash_core::AppError) -> bool {
    error.user_message() == NOT_SMALLER_MESSAGE
}

#[cfg(test)]
mod tests;
