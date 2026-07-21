use std::path::Path;

use dropsquash_core::{AppError, MaskPlan, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{sha256_hex_for_file, verify_evidence_signature, EvidenceSignature};

mod native_evidence;
mod strict_plan;

const CANONICAL_EVIDENCE_SCHEMA_VERSION: u32 = 2;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecureShareEvidence {
    pub schema_version: u32,
    pub output_name: String,
    pub output_sha256: String,
    pub plan: MaskPlan,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SignedSecureShareEvidence {
    pub evidence: SecureShareEvidence,
    pub signature: EvidenceSignature,
}

impl SignedSecureShareEvidence {
    pub fn signature_payload(&self) -> Result<Vec<u8>> {
        canonical_payload(&self.evidence)
    }

    pub fn verify_signature(&self) -> Result<()> {
        verify_evidence_signature(&self.signature_payload()?, &self.signature)
    }
}

pub fn canonical_payload(evidence: &SecureShareEvidence) -> Result<Vec<u8>> {
    Ok(serde_json::to_vec(&serde_json::to_value(evidence)?)?)
}

pub fn verify_secure_share_evidence(
    video_path: &Path,
    sidecar_path: &Path,
) -> Result<SecureShareEvidence> {
    let bytes = std::fs::read(sidecar_path)?;
    reject_private_fields(&serde_json::from_slice(&bytes)?)?;
    let signed: SignedSecureShareEvidence = serde_json::from_slice(&bytes)?;
    if signed.evidence.schema_version != CANONICAL_EVIDENCE_SCHEMA_VERSION {
        return Err(invalid("uses a legacy or unsupported schema version"));
    }
    signed.verify_signature()?;
    strict_plan::require_redacted_strict_plan(&signed.evidence.plan)?;
    verify_video_binding(video_path, &signed.evidence)?;
    Ok(signed.evidence)
}

fn verify_video_binding(video_path: &Path, evidence: &SecureShareEvidence) -> Result<()> {
    let output_name = video_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| invalid("video path has no UTF-8 file name"))?;
    if evidence.output_name != output_name {
        return Err(invalid("output name does not match the video"));
    }
    if evidence.output_sha256 != sha256_hex_for_file(video_path)? {
        return Err(invalid("output SHA-256 does not match the video"));
    }
    Ok(())
}

fn reject_private_fields(value: &Value) -> Result<()> {
    match value {
        Value::Object(fields) => {
            for (key, child) in fields {
                if matches!(
                    key.as_str(),
                    "text"
                        | "recognizedText"
                        | "accessibility"
                        | "vision"
                        | "focusedValue"
                        | "windowTitle"
                        | "applicationName"
                        | "bundleIdentifier"
                        | "notificationPayload"
                        | "rawPixels"
                        | "screenshot"
                ) {
                    return Err(invalid("contains a raw private observation field"));
                }
                reject_private_fields(child)?;
            }
        }
        Value::Array(items) => {
            for item in items {
                reject_private_fields(item)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn invalid(reason: &str) -> AppError {
    AppError::InvalidConfig(format!("Secure Share evidence {reason}"))
}

#[cfg(test)]
mod tests;
