use dropsquash_core::{AppConfig, EncodeResult, LicenseState, OutputSize, Profile, SourcePolicy};
use serde::Serialize;

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

impl From<EncodeResult> for ConversionSummary {
    fn from(result: EncodeResult) -> Self {
        Self {
            output_path: result.output_path.display().to_string(),
            original_bytes: result.original_bytes,
            output_bytes: result.output_bytes,
            saved_bytes: result.saved_bytes(),
            reduction_percent: result.reduction_percent(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedConfig {
    pub output_dir: String,
    pub profile: Profile,
    pub output_size: OutputSize,
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
        profiles: select_options(Profile::DELIVERY),
        output_sizes: size_options(OutputSize::ALL),
        input_extensions,
        source_policy: config.source_policy,
        privacy_mode: "local-only",
        successful_conversions: trial_state.successful_conversions,
        trial_limit: trial_state.limit,
        is_locked: trial_state.is_locked(),
    }
}

fn select_options(values: [Profile; 8]) -> Vec<ProfileOption> {
    values
        .into_iter()
        .map(|value| ProfileOption {
            value,
            label: value.display_name(),
        })
        .collect()
}

fn size_options(values: [OutputSize; 4]) -> Vec<OutputSizeOption> {
    values
        .into_iter()
        .map(|value| OutputSizeOption {
            value,
            label: value.display_name(),
        })
        .collect()
}
