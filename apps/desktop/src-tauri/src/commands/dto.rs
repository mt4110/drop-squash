mod options;
mod summary;

pub use summary::ConversionSummary;

use dropsquash_core::{AppConfig, LicenseState, OutputSize, Profile, SourcePolicy};
use serde::{Deserialize, Serialize};

use options::{
    profile_options, size_options, source_policy_options, OutputSizeOption, ProfileOption,
    SourcePolicyOption,
};

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
    pub source_policies: Vec<SourcePolicyOption>,
    pub privacy_mode: &'static str,
    pub write_privacy_receipt: bool,
    pub successful_conversions: u32,
    pub trial_limit: u32,
    pub is_pro: bool,
    pub is_locked: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedConfig {
    pub output_dir: String,
    pub profile: Profile,
    pub output_size: OutputSize,
    pub source_policy: SourcePolicy,
    pub write_privacy_receipt: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertRequest {
    pub input_path: String,
    pub output_dir: String,
    pub profile: Profile,
    pub output_size: OutputSize,
    pub source_policy: SourcePolicy,
    pub write_privacy_receipt: bool,
}

pub fn drop_zone_state(
    config: &AppConfig,
    license_state: LicenseState,
    input_extensions: Vec<String>,
) -> DropZoneState {
    let is_pro = matches!(license_state, LicenseState::Pro);
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
        profiles: profile_options(Profile::DELIVERY),
        output_sizes: size_options(OutputSize::ALL),
        input_extensions,
        source_policy: config.source_policy,
        source_policies: source_policy_options(),
        privacy_mode: "local-only",
        write_privacy_receipt: config.write_privacy_receipt,
        successful_conversions: trial_state.successful_conversions,
        trial_limit: trial_state.limit,
        is_pro,
        is_locked: trial_state.is_locked(),
    }
}
