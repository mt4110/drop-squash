use dropsquash_core::{
    EncodeResult, FrameSize, MaskMode, MaskPlan, MaskPlanAudit, MaskPolicy, MaskReason, MaskRect,
    ObservationSource, Profile, SecureShareOptions, TimeRangeNs, VerificationExpectations,
};

use super::save_receipt;

#[test]
fn keeps_privacy_receipt_for_regular_results() {
    let dir = tempfile::tempdir().unwrap();
    let result = fixture(dir.path().join("out.mp4"));
    std::fs::write(&result.output_path, b"regular").unwrap();

    let saved = save_receipt(&result, None, None, true).unwrap().unwrap();

    assert_eq!(saved.kind, "privacy");
    assert!(saved.path.ends_with("out.privacy.json"));
}

#[test]
fn switches_to_secure_share_receipt_when_options_exist() {
    let dir = tempfile::tempdir().unwrap();
    let result = fixture(dir.path().join("out.mp4"));
    std::fs::write(&result.output_path, b"secure-share").unwrap();

    let saved = save_receipt(&result, Some(&options()), None, false)
        .unwrap()
        .unwrap();

    assert_eq!(saved.kind, "secure-share");
    assert!(saved.path.ends_with("out.secure-share.json"));
}

#[test]
fn secure_share_receipt_can_include_mask_plan_audit() {
    let dir = tempfile::tempdir().unwrap();
    let result = fixture(dir.path().join("out.mp4"));
    std::fs::write(&result.output_path, b"secure-share").unwrap();

    let saved = save_receipt(&result, Some(&options()), Some(&mask_plan()), false)
        .unwrap()
        .unwrap();
    let json = std::fs::read_to_string(saved.path).unwrap();

    assert!(json.contains("\"mask_plan_audit\""));
    assert!(json.contains("\"verification_required_frame_count\": 1"));
}

fn fixture(output_path: std::path::PathBuf) -> EncodeResult {
    EncodeResult {
        input_path: "input.mov".into(),
        output_path,
        profile: Profile::Auto,
        original_bytes: 100,
        output_bytes: 40,
        success: true,
        error_message: None,
    }
}

fn options() -> SecureShareOptions {
    SecureShareOptions {
        mask_mode: MaskMode::SolidBlack,
        mask_rects: vec![MaskRect {
            x: 1,
            y: 2,
            width: 3,
            height: 4,
        }],
        mask_plan: None,
    }
}

fn mask_plan() -> MaskPlan {
    let mut audit = MaskPlanAudit::clean();
    audit.record_unmatched(
        ObservationSource::AccessibilityText,
        MaskReason::AxTextElement,
        TimeRangeNs {
            start_ns: 0,
            end_ns: 1,
        },
    );
    audit.verification_required_frame_count = 1;
    MaskPlan {
        schema_version: 1,
        capture_id: "desktop-receipt-test".to_string(),
        frame_size: FrameSize {
            width: 1280,
            height: 720,
        },
        frames: Vec::new(),
        policy: MaskPolicy::StrictReveal,
        audit,
        verification_expectations: VerificationExpectations {
            no_audio: true,
            strip_metadata: true,
            verification_policy_version: "secure-share-rd-1".to_string(),
        },
    }
}
