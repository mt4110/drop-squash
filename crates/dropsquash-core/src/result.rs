use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::Profile;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncodeResult {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub profile: Profile,
    pub original_bytes: u64,
    pub output_bytes: u64,
    pub success: bool,
    pub error_message: Option<String>,
}

impl EncodeResult {
    pub fn is_successful_conversion(&self) -> bool {
        self.success
            && self.original_bytes > 0
            && self.output_bytes > 0
            && self.output_bytes < self.original_bytes
    }

    pub fn saved_bytes(&self) -> u64 {
        self.original_bytes.saturating_sub(self.output_bytes)
    }

    pub fn reduction_percent(&self) -> f64 {
        if self.original_bytes == 0 {
            return 0.0;
        }

        self.saved_bytes() as f64 / self.original_bytes as f64 * 100.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LicenseState {
    Trial(TrialState),
    Pro,
    Locked {
        trial: TrialState,
        reason: LockedReason,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LockedReason {
    TrialComplete,
    LicenseRefreshRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrialState {
    pub successful_conversions: u32,
    pub limit: u32,
}

impl TrialState {
    pub fn remaining(&self) -> u32 {
        self.limit.saturating_sub(self.successful_conversions)
    }

    pub fn is_locked(&self) -> bool {
        self.successful_conversions >= self.limit
    }
}
