use std::path::PathBuf;

use dropsquash_core::{
    default_history_path, default_license_cache_path, AppError, EncodeJob, LicenseState,
    LockedReason, MaskPlan, SecureShareOptions, SourcePolicy, NOT_SMALLER_MESSAGE,
};
use dropsquash_encoder::{resolve_secure_share_options, EncoderBackend};
use dropsquash_history::{append_successful_record, read_records, HistoryMetrics};
use dropsquash_privacy::{PrivacyReceipt, SecureShareReceipt};

use crate::args::{OutputSizeArg, ProfileArg};

use super::license;

#[cfg(target_os = "macos")]
use dropsquash_encoder::AppleNativeEncoder as NativeEncoder;
#[cfg(target_os = "linux")]
use dropsquash_encoder::GStreamerEncoder as NativeEncoder;
#[cfg(target_os = "windows")]
use dropsquash_encoder::MediaFoundationEncoder as NativeEncoder;

pub async fn run(
    input: PathBuf,
    output_dir: PathBuf,
    profile: ProfileArg,
    output_size: OutputSizeArg,
    history: Option<PathBuf>,
    secure_share: Option<SecureShareOptions>,
) -> dropsquash_core::Result<()> {
    let history = history.unwrap_or_else(default_history_path);
    ensure_trial_available(&history).await?;
    let secure_share = resolve_secure_share_options(&input, secure_share.as_ref())?;
    let result = match NativeEncoder
        .encode(EncodeJob {
            input_path: input,
            output_dir,
            profile: profile.into(),
            output_size: output_size.into(),
            source_policy: SourcePolicy::Ask,
            secure_share: secure_share.clone(),
        })
        .await
    {
        Ok(result) => result,
        Err(error) if is_not_smaller_error(&error) => {
            println!("{NOT_SMALLER_MESSAGE}");
            license::print_state(license::state(&history, &default_license_cache_path()).await?);
            return Ok(());
        }
        Err(error) => return Err(error),
    };
    let receipt_path = save_receipt(&result, secure_share.as_ref(), None)?;
    append_successful_record(&history, result.clone()).await?;
    println!("output: {}", result.output_path.display());
    println!("receipt: {}", receipt_path.display());
    println!("original bytes: {}", result.original_bytes);
    println!("squashed bytes: {}", result.output_bytes);
    println!("saved bytes: {}", result.saved_bytes());
    println!("reduction: {:.2}%", result.reduction_percent());
    license::print_state(license::state(&history, &default_license_cache_path()).await?);
    Ok(())
}

async fn ensure_trial_available(history: &std::path::Path) -> dropsquash_core::Result<()> {
    let records = read_records(history).await?;
    let metrics = HistoryMetrics::from_records(&records);
    if let LicenseState::Locked { reason, .. } = license::gate()?.state_for_metrics(metrics) {
        return Err(AppError::License(locked_message(reason).to_string()));
    }
    Ok(())
}

fn locked_message(reason: LockedReason) -> &'static str {
    match reason {
        LockedReason::TrialComplete => "trial limit reached; enter a license key to continue",
        LockedReason::LicenseRefreshRequired => {
            "reconnect once with your license key to refresh Pro"
        }
    }
}

fn is_not_smaller_error(error: &AppError) -> bool {
    error.user_message() == NOT_SMALLER_MESSAGE
}

fn save_receipt(
    result: &dropsquash_core::EncodeResult,
    secure_share: Option<&dropsquash_core::SecureShareOptions>,
    mask_plan: Option<&MaskPlan>,
) -> dropsquash_core::Result<PathBuf> {
    if let Some(options) = secure_share {
        return match mask_plan {
            Some(plan) => SecureShareReceipt::save_for_result_with_mask_plan(result, options, plan),
            None => SecureShareReceipt::save_for_result(result, options),
        };
    }
    PrivacyReceipt::save_for_result(result)
}

#[cfg(test)]
mod tests;
