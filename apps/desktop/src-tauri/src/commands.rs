use std::{path::PathBuf, sync::Arc};

use dropsquash_core::{
    default_history_path, AppConfig, AppError, EncodeJob, LicenseState, OutputSize, Profile,
    SourcePolicy, TRIAL_CONVERSION_LIMIT,
};
use dropsquash_encoder::{EncodeProgressReporter, EncoderBackend};
use dropsquash_fileguard::{wait_until_stable, StabilityOptions};
use dropsquash_history::{append_record, read_records, ConversionRecord, HistoryMetrics};
use dropsquash_license::LicenseGate;
use serde::Serialize;
use tauri::Emitter;
use tokio_util::sync::CancellationToken;

#[cfg(target_os = "linux")]
use dropsquash_encoder::GStreamerEncoder as NativeEncoder;
#[cfg(target_os = "windows")]
use dropsquash_encoder::MediaFoundationEncoder as NativeEncoder;
#[cfg(target_os = "macos")]
use dropsquash_encoder::VideoToolboxEncoder as NativeEncoder;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DropZoneState {
    pub product_name: &'static str,
    pub output_dir: String,
    pub profile: Profile,
    pub output_size: OutputSize,
    pub profiles: Vec<ProfileOption>,
    pub output_sizes: Vec<OutputSizeOption>,
    pub input_extensions: Vec<String>,
    pub source_policy: SourcePolicy,
    pub privacy_mode: &'static str,
    pub successful_conversions: u32,
    pub trial_limit: u32,
    pub is_locked: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileOption {
    pub value: Profile,
    pub label: &'static str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputSizeOption {
    pub value: OutputSize,
    pub label: &'static str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversionSummary {
    pub output_path: String,
    pub original_bytes: u64,
    pub output_bytes: u64,
    pub saved_bytes: u64,
    pub reduction_percent: f64,
}

#[derive(Clone)]
struct WindowProgressReporter {
    window: tauri::WebviewWindow,
}

impl EncodeProgressReporter for WindowProgressReporter {
    fn report(&self, fraction: f32) {
        let percent = (fraction.clamp(0.0, 1.0) * 100.0).round() as u8;
        let _ = self.window.emit("conversion-progress", percent);
    }
}

pub fn drop_zone_state(
    config: &AppConfig,
    license_state: LicenseState,
    input_extensions: Vec<String>,
) -> DropZoneState {
    let trial_state = match license_state {
        LicenseState::Trial(state) | LicenseState::Locked(state) => state,
        LicenseState::Pro => dropsquash_core::TrialState {
            successful_conversions: 0,
            limit: config.trial_conversion_limit,
        },
    };

    DropZoneState {
        product_name: "DropSquash",
        output_dir: config.output_dir.display().to_string(),
        profile: config.default_profile,
        output_size: config.default_output_size,
        profiles: Profile::DELIVERY
            .into_iter()
            .map(|value| ProfileOption {
                value,
                label: value.display_name(),
            })
            .collect(),
        output_sizes: OutputSize::ALL
            .into_iter()
            .map(|value| OutputSizeOption {
                value,
                label: value.display_name(),
            })
            .collect(),
        input_extensions,
        source_policy: config.source_policy,
        privacy_mode: "local-only",
        successful_conversions: trial_state.successful_conversions,
        trial_limit: trial_state.limit,
        is_locked: trial_state.is_locked(),
    }
}

#[tauri::command(rename_all = "camelCase")]
pub async fn load_state() -> std::result::Result<DropZoneState, String> {
    let config = AppConfig::default();
    let license_state = current_license_state().await.map_err(format_error)?;
    let capabilities = NativeEncoder.probe_capabilities().map_err(format_error)?;

    Ok(drop_zone_state(
        &config,
        license_state,
        capabilities.input_extensions,
    ))
}

#[tauri::command(rename_all = "camelCase")]
pub async fn convert(
    window: tauri::WebviewWindow,
    input_path: String,
    output_dir: String,
    profile: Profile,
    output_size: OutputSize,
) -> std::result::Result<ConversionSummary, String> {
    let license_state = current_license_state().await.map_err(format_error)?;
    if matches!(license_state, LicenseState::Locked(_)) {
        return Err("Trial complete. Enter a license key to continue.".to_string());
    }

    let input_path = PathBuf::from(input_path);
    let capabilities = NativeEncoder.probe_capabilities().map_err(format_error)?;
    let input_extension = input_path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase)
        .unwrap_or_default();
    if !capabilities
        .input_extensions
        .iter()
        .any(|extension| extension == &input_extension)
    {
        return Err(format_error(AppError::UnsupportedMedia(input_extension)));
    }
    wait_until_stable(
        &input_path,
        StabilityOptions::default(),
        CancellationToken::new(),
    )
    .await
    .map_err(format_error)?;

    let result = NativeEncoder
        .encode_with_progress(
            EncodeJob {
                input_path,
                output_dir: PathBuf::from(output_dir),
                profile,
                output_size,
                source_policy: SourcePolicy::Ask,
            },
            Arc::new(WindowProgressReporter { window }),
        )
        .await
        .map_err(format_error)?;
    append_record(
        &default_history_path(),
        &ConversionRecord::new(result.clone()),
    )
    .await
    .map_err(format_error)?;

    Ok(ConversionSummary {
        output_path: result.output_path.display().to_string(),
        original_bytes: result.original_bytes,
        output_bytes: result.output_bytes,
        saved_bytes: result.saved_bytes(),
        reduction_percent: result.reduction_percent(),
    })
}

async fn current_license_state() -> dropsquash_core::Result<LicenseState> {
    let records = read_records(&default_history_path()).await?;
    let metrics = HistoryMetrics::from_records(&records);

    Ok(LicenseGate {
        trial_limit: TRIAL_CONVERSION_LIMIT,
        has_valid_license: false,
    }
    .state_for_metrics(metrics))
}

fn format_error(error: AppError) -> String {
    error.to_string()
}

#[cfg(test)]
mod tests {
    use dropsquash_core::AppConfig;

    use super::*;

    #[test]
    fn exposes_placeholder_drop_zone_state() {
        let state = drop_zone_state(
            &AppConfig::default(),
            LicenseState::Trial(dropsquash_core::TrialState {
                successful_conversions: 0,
                limit: TRIAL_CONVERSION_LIMIT,
            }),
            vec!["mov".to_string(), "mp4".to_string(), "m4v".to_string()],
        );
        assert_eq!(state.product_name, "DropSquash");
        assert_eq!(state.privacy_mode, "local-only");
        assert_eq!(state.output_size, OutputSize::Auto);
        assert_eq!(state.profiles.len(), Profile::DELIVERY.len());
        assert_eq!(state.output_sizes.len(), OutputSize::ALL.len());
        assert_eq!(state.input_extensions, ["mov", "mp4", "m4v"]);
        assert!(!state.is_locked);
    }
}
