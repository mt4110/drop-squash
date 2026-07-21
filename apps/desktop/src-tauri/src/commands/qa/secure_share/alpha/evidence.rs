use std::path::{Path, PathBuf};

use dropsquash_core::{AppError, MaskPlan, Result};
use dropsquash_fileguard::{
    canonical_payload, sha256_hex_for_file, sign_evidence, SecureShareEvidence,
    SignedSecureShareEvidence,
};

pub(crate) fn prepare(
    partial_video: &Path,
    final_video: &Path,
    plan: &MaskPlan,
    key_path: &Path,
) -> Result<PathBuf> {
    let output_name = final_video
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| AppError::InvalidConfig("Secure Share output name is invalid".into()))?
        .to_string();
    let evidence = SecureShareEvidence {
        schema_version: 2,
        output_name,
        output_sha256: sha256_hex_for_file(partial_video)?,
        plan: plan.clone(),
    };
    let signature = sign_evidence(&canonical_payload(&evidence)?, key_path)?;
    let signed = SignedSecureShareEvidence {
        evidence,
        signature,
    };
    let temporary = temporary_path(final_video);
    let bytes = serde_json::to_vec_pretty(&signed)?;
    if let Err(error) = std::fs::write(&temporary, bytes) {
        let _ = std::fs::remove_file(&temporary);
        return Err(error.into());
    }
    let verified = std::fs::read(&temporary)
        .map_err(AppError::from)
        .and_then(|bytes| Ok(serde_json::from_slice::<SignedSecureShareEvidence>(&bytes)?))
        .and_then(|value| value.verify_signature());
    if let Err(error) = verified {
        let _ = std::fs::remove_file(&temporary);
        return Err(error);
    }
    Ok(temporary)
}

pub(crate) fn publish(temporary: &Path, final_video: &Path) -> Result<PathBuf> {
    let final_path = final_path(final_video);
    std::fs::rename(temporary, &final_path)?;
    Ok(final_path)
}

pub(crate) fn discard(temporary: &Path, final_video: &Path) {
    let _ = std::fs::remove_file(temporary);
    let _ = std::fs::remove_file(final_path(final_video));
}

fn temporary_path(final_video: &Path) -> PathBuf {
    final_path(final_video).with_extension("json.partial")
}

fn final_path(final_video: &Path) -> PathBuf {
    final_video.with_extension("mask-plan.json")
}

#[cfg(test)]
mod tests;
