use dropsquash_core::{FrameSize, MaskPlan, MaskPlanAudit, MaskPolicy, VerificationExpectations};

use super::precision;

#[test]
fn rejects_truth_with_a_different_frame_size() {
    let file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(
        file.path(),
        r#"{"frameWidth":2,"frameHeight":1,"sensitive":[{"x":0,"y":0,"width":1,"height":1}]}"#,
    )
    .unwrap();
    unsafe { std::env::set_var("DROP_SQUASH_QA_SENSITIVE_TRUTH_PATH", file.path()) };
    let error = precision(&plan()).unwrap_err().to_string();
    unsafe { std::env::remove_var("DROP_SQUASH_QA_SENSITIVE_TRUTH_PATH") };
    assert!(error.contains("frame size differs"));
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
