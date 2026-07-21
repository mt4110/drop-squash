use dropsquash_core::{
    AppError, EncodeResult, FrameSize, MaskMode, MaskPlan, MaskPlanAudit, MaskPolicy, MaskReason,
    MaskRect, ObservationSource, Profile, SecureShareOptions, TimeRangeNs,
    VerificationExpectations,
};

use super::{is_not_smaller_error, save_receipt};

#[test]
fn recognizes_raw_not_smaller_encoder_errors() {
    assert!(is_not_smaller_error(&AppError::Encoder(
        "native export failed output verification: output is not smaller (1 bytes -> 2 bytes)"
            .to_string(),
    )));
}

#[test]
fn recognizes_friendly_not_smaller_encoder_errors() {
    assert!(is_not_smaller_error(&AppError::Encoder(
        "This recording could not be made smaller. It may already be small, so DropSquash kept the original and did not count the attempt. Try a smaller Size setting for this clip."
            .to_string(),
    )));
}

#[test]
fn ignores_other_encoder_failures() {
    assert!(!is_not_smaller_error(&AppError::Encoder(
        "The operation could not be completed".to_string(),
    )));
}

#[test]
fn secure_share_receipt_can_include_mask_plan_audit() {
    let dir = tempfile::tempdir().unwrap();
    let result = encode_result(dir.path().join("out.mp4"));
    std::fs::write(&result.output_path, b"secure-share").unwrap();

    let receipt_path = save_receipt(&result, Some(&options()), Some(&mask_plan())).unwrap();
    let json = std::fs::read_to_string(receipt_path).unwrap();

    assert!(json.contains("\"mask_plan_audit\""));
    assert!(json.contains("\"unmatched_observation_count\": 1"));
}

fn encode_result(output_path: std::path::PathBuf) -> EncodeResult {
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
        ObservationSource::VisionTextRecognition,
        MaskReason::VisionText,
        TimeRangeNs {
            start_ns: 0,
            end_ns: 1,
        },
    );
    audit.verification_required_frame_count = 1;
    MaskPlan {
        schema_version: 1,
        capture_id: "cli-receipt-test".to_string(),
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
