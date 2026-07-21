use std::fs;

use dropsquash_core::{
    FrameMaskPlan, FrameSize, FrameStatus, MaskPlan, MaskPlanAudit, MaskPolicy,
    VerificationExpectations,
};
use serde_json::Value;

use super::{discard, prepare};
use dropsquash_fileguard::SignedSecureShareEvidence;

#[test]
fn rejects_a_corrupt_signing_key_before_writing_evidence() {
    let directory = tempfile::tempdir().unwrap();
    let partial = directory.path().join("clip.partial.mp4");
    let final_video = directory.path().join("clip.mp4");
    let key = directory.path().join("corrupt.pk8");
    fs::write(&partial, b"video").unwrap();
    fs::write(&key, b"not a private key").unwrap();

    assert!(prepare(&partial, &final_video, &plan(), &key).is_err());
    assert!(!final_video
        .with_extension("mask-plan.json.partial")
        .exists());
}

#[test]
fn discard_removes_both_evidence_paths() {
    let directory = tempfile::tempdir().unwrap();
    let video = directory.path().join("clip.mp4");
    let temporary = video.with_extension("mask-plan.json.partial");
    let final_path = video.with_extension("mask-plan.json");
    fs::write(&temporary, b"temporary").unwrap();
    fs::write(&final_path, b"published").unwrap();

    discard(&temporary, &video);
    assert!(!temporary.exists());
    assert!(!final_path.exists());
}

#[test]
fn evidence_keeps_raw_observations_and_recognized_text_out_of_the_sidecar() {
    let directory = tempfile::tempdir().unwrap();
    let partial = directory.path().join("clip.partial.mp4");
    let final_video = directory.path().join("clip.mp4");
    let key = directory.path().join("evidence.pk8");
    fs::write(&partial, b"video").unwrap();

    let temporary = prepare(&partial, &final_video, &plan(), &key).unwrap();
    let evidence: Value = serde_json::from_slice(&fs::read(&temporary).unwrap()).unwrap();
    let plan = &evidence["evidence"]["plan"];

    assert!(plan.get("accessibility").is_none());
    assert!(plan.get("vision").is_none());
    assert!(!evidence.to_string().contains("\"text\""));
}

#[test]
fn written_evidence_verifies_after_a_json_round_trip() {
    let directory = tempfile::tempdir().unwrap();
    let partial = directory.path().join("clip.partial.mp4");
    let final_video = directory.path().join("clip.mp4");
    let key = directory.path().join("evidence.pk8");
    fs::write(&partial, b"video").unwrap();

    let temporary = prepare(&partial, &final_video, &plan(), &key).unwrap();
    let signed: SignedSecureShareEvidence =
        serde_json::from_slice(&fs::read(temporary).unwrap()).unwrap();

    signed.verify_signature().unwrap();
}

fn plan() -> MaskPlan {
    MaskPlan {
        schema_version: 1,
        capture_id: "test".into(),
        frame_size: FrameSize {
            width: 1,
            height: 1,
        },
        frames: vec![FrameMaskPlan {
            frame_index: 0,
            presentation_time_ns: 0,
            frame_status: FrameStatus::Complete,
            regions: Vec::new(),
        }],
        policy: MaskPolicy::StrictReveal,
        audit: MaskPlanAudit::clean(),
        verification_expectations: VerificationExpectations {
            no_audio: true,
            strip_metadata: true,
            verification_policy_version: "test".into(),
        },
    }
}
