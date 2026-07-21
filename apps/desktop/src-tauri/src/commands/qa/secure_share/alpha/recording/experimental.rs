use std::path::{Path, PathBuf};

use dropsquash_core::{MaskPlan, MaskPolicy, Result};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ExperimentalArtifact<'a> {
    artifact_kind: &'static str,
    shareable: bool,
    plan: &'a MaskPlan,
    decoded_text_observation_count: u64,
    decoded_unmasked_text_count: u64,
}

pub(super) fn save(
    video: &Path,
    plan: &MaskPlan,
    decoded_text_observation_count: u64,
    decoded_unmasked_text_count: u64,
) -> Result<PathBuf> {
    let path = video.with_extension("mask-plan.experimental.json");
    let artifact = ExperimentalArtifact {
        artifact_kind: artifact_kind(plan.policy),
        shareable: false,
        plan,
        decoded_text_observation_count,
        decoded_unmasked_text_count,
    };
    std::fs::write(&path, serde_json::to_vec_pretty(&artifact)?)?;
    Ok(path)
}

fn artifact_kind(policy: MaskPolicy) -> &'static str {
    match policy {
        MaskPolicy::SmartMask => "secure_share_smart_mask_experiment",
        MaskPolicy::StrictReveal => "secure_share_strict_shield_experiment",
    }
}

#[cfg(test)]
mod tests {
    use dropsquash_core::{FrameSize, MaskPlanAudit, MaskPolicy, VerificationExpectations};

    use super::*;

    #[test]
    fn writes_an_explicitly_labeled_plan() {
        let directory = tempfile::tempdir().unwrap();
        let path = save(&directory.path().join("clip.mp4"), &plan(), 4, 0).unwrap();

        assert_eq!(
            path.file_name().unwrap(),
            "clip.mask-plan.experimental.json"
        );
        assert!(path.is_file());
        let text = std::fs::read_to_string(path).unwrap();
        assert!(text.contains("secure_share_smart_mask_experiment"));
        assert!(text.contains("\"shareable\": false"));
        assert!(text.contains("decodedTextObservationCount"));
    }

    fn plan() -> MaskPlan {
        MaskPlan {
            schema_version: 1,
            capture_id: "fixture".into(),
            frame_size: FrameSize {
                width: 1,
                height: 1,
            },
            frames: Vec::new(),
            policy: MaskPolicy::SmartMask,
            audit: MaskPlanAudit::clean(),
            verification_expectations: VerificationExpectations {
                no_audio: true,
                strip_metadata: true,
                verification_policy_version: "fixture".into(),
            },
        }
    }
}
