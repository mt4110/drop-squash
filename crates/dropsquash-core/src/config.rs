use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{default_output_dir, OutputSize, Profile, SourcePolicy};

pub const TRIAL_CONVERSION_LIMIT: u32 = 10;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppConfig {
    pub output_dir: PathBuf,
    pub default_profile: Profile,
    pub default_output_size: OutputSize,
    pub source_policy: SourcePolicy,
    pub trial_conversion_limit: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            output_dir: default_output_dir(),
            default_profile: Profile::Auto,
            default_output_size: OutputSize::Auto,
            source_policy: SourcePolicy::Ask,
            trial_conversion_limit: TRIAL_CONVERSION_LIMIT,
        }
    }
}
