mod io;

use std::path::Path;

use dropsquash_core::{EncodeResult, MaskPlan, MaskReason, SecureShareOptions};
use dropsquash_fileguard::sha256_hex_for_file;
use serde::{Deserialize, Serialize};

use crate::MetadataPolicy;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecureShareReceipt {
    pub input_name: String,
    pub output_name: String,
    pub output_sha256: String,
    pub mask_mode: String,
    pub mask_rect_count: usize,
    pub exported_at_utc: u64,
    pub metadata_policy: MetadataPolicy,
    pub mask_plan_audit: Option<SecureShareAuditSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecureShareAuditSummary {
    pub unmatched_observation_count: usize,
    pub unmatched_reasons: Vec<MaskReason>,
    pub verification_required_frame_count: usize,
}

impl SecureShareReceipt {
    pub fn from_result(
        result: &EncodeResult,
        options: &SecureShareOptions,
    ) -> dropsquash_core::Result<Self> {
        Ok(Self {
            input_name: file_name(&result.input_path),
            output_name: file_name(&result.output_path),
            output_sha256: sha256_hex_for_file(&result.output_path)?,
            mask_mode: options.mask_mode.as_str().to_string(),
            mask_rect_count: options.mask_rects.len(),
            exported_at_utc: unix_seconds_now(),
            metadata_policy: MetadataPolicy::Preserve,
            mask_plan_audit: None,
        })
    }

    pub fn from_result_with_mask_plan(
        result: &EncodeResult,
        options: &SecureShareOptions,
        mask_plan: &MaskPlan,
    ) -> dropsquash_core::Result<Self> {
        let mut receipt = Self::from_result(result, options)?;
        receipt.mask_plan_audit = Some(SecureShareAuditSummary::from(mask_plan));
        Ok(receipt)
    }
}

impl From<&MaskPlan> for SecureShareAuditSummary {
    fn from(mask_plan: &MaskPlan) -> Self {
        Self {
            unmatched_observation_count: mask_plan.audit.unmatched_observations.len(),
            unmatched_reasons: mask_plan
                .audit
                .unmatched_observations
                .iter()
                .map(|observation| observation.reason)
                .collect(),
            verification_required_frame_count: mask_plan.audit.verification_required_frame_count,
        }
    }
}

fn file_name(path: &Path) -> String {
    path.file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_string()
}

fn unix_seconds_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|value| value.as_secs())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests;
