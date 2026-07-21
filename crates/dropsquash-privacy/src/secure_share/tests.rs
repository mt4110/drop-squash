use std::path::PathBuf;

use dropsquash_core::{
    CaptureFrameMetadata, CaptureRect, Confidence, EncodeResult, FrameSize, FrameStatus, MaskMode,
    MaskPlan, MaskPlanAudit, MaskPolicy, MaskReason, MaskRect, ObservationSource, PixelRect,
    Profile, SecureShareOptions, TimeRangeNs, VerificationExpectations,
};
use dropsquash_platform::SecureShareObservationSnapshot;

use super::{io::receipt_path_for, SecureShareReceipt};
use crate::MetadataPolicy;

#[test]
fn receipt_json_stays_bounded_and_local_only() {
    let directory = tempfile::tempdir().unwrap();
    let output_path = directory.path().join("out.mp4");
    std::fs::write(&output_path, b"share").unwrap();
    let result = encode_result("/Users/me/Secret/input.mov", &output_path);

    let receipt = SecureShareReceipt::from_result(&result, &options()).unwrap();
    let json = serde_json::to_string(&receipt).unwrap();

    assert!(json.contains("\"input_name\":\"input.mov\""));
    assert!(json.contains("\"output_name\":\"out.mp4\""));
    assert!(json.contains("\"mask_mode\":\"solid_black\""));
    assert!(json.contains("\"mask_rect_count\":1"));
    assert!(json.contains("\"metadata_policy\":\"preserve\""));
    assert!(json.contains("\"mask_plan_audit\":null"));
    assert!(!json.contains("/Users/me/Secret"));
}

#[test]
fn receipt_can_summarize_mask_plan_audit_without_private_text() {
    let directory = tempfile::tempdir().unwrap();
    let output_path = directory.path().join("out.mp4");
    std::fs::write(&output_path, b"share").unwrap();
    let result = encode_result("/Users/me/Secret/input.mov", &output_path);

    let receipt =
        SecureShareReceipt::from_result_with_mask_plan(&result, &options(), &mask_plan()).unwrap();
    let json = serde_json::to_string(&receipt).unwrap();

    assert!(json.contains("\"unmatched_observation_count\":1"));
    assert!(json.contains("\"unmatched_reasons\":[\"vision_text\"]"));
    assert!(json.contains("\"verification_required_frame_count\":2"));
    assert!(!json.contains("/Users/me/Secret"));
    assert!(!json.contains("customer@example.com"));
}

#[test]
fn snapshot_mask_plan_audit_flows_into_receipt() {
    let directory = tempfile::tempdir().unwrap();
    let output_path = directory.path().join("out.mp4");
    std::fs::write(&output_path, b"share").unwrap();
    let result = encode_result("/Users/me/Secret/input.mov", &output_path);
    let plan = unmatched_snapshot().into_mask_plan(
        "capture-receipt-snapshot-001".to_string(),
        MaskPolicy::StrictReveal,
        verification(),
    );

    let receipt =
        SecureShareReceipt::from_result_with_mask_plan(&result, &options(), &plan).unwrap();

    let audit = receipt.mask_plan_audit.unwrap();
    assert_eq!(audit.unmatched_observation_count, 0);
    assert!(audit.unmatched_reasons.is_empty());
    assert_eq!(audit.verification_required_frame_count, 0);
}

#[test]
fn receipt_path_replaces_output_extension() {
    assert_eq!(
        receipt_path_for(&PathBuf::from("recording.squashed.mp4")),
        PathBuf::from("recording.squashed.secure-share.json")
    );
}

#[test]
fn save_for_result_writes_hash_and_receipt_fields() {
    let directory = tempfile::tempdir().unwrap();
    let output_path = directory.path().join("recording.squashed.mp4");
    std::fs::write(&output_path, b"share").unwrap();
    let result = encode_result("input.mov", &output_path);

    let receipt_path = SecureShareReceipt::save_for_result(&result, &options()).unwrap();
    let json = std::fs::read_to_string(receipt_path).unwrap();

    assert!(json.contains(
        "\"output_sha256\": \"c3bc45ac352fe43ff8f0a1cc26d6cc29f71f536dc417906f8513ea44ed4bb161\""
    ));
    assert!(json.contains("\"mask_mode\": \"solid_black\""));
    assert!(json.contains("\"mask_rect_count\": 1"));
}

#[test]
fn load_for_output_reads_matching_sidecar() {
    let directory = tempfile::tempdir().unwrap();
    let output_path = directory.path().join("recording.squashed.mp4");
    std::fs::write(&output_path, b"share").unwrap();
    let result = encode_result("input.mov", &output_path);
    let saved = SecureShareReceipt::save_for_result(&result, &options()).unwrap();

    let (loaded_path, receipt) = SecureShareReceipt::load_for_output(&output_path).unwrap();

    assert_eq!(loaded_path, saved);
    assert_eq!(receipt.output_name, "recording.squashed.mp4");
    assert_eq!(receipt.mask_rect_count, 1);
    assert_eq!(receipt.metadata_policy, MetadataPolicy::Preserve);
    assert_eq!(receipt.mask_plan_audit, None);
}

fn encode_result(input: impl Into<PathBuf>, output: impl Into<PathBuf>) -> EncodeResult {
    EncodeResult {
        input_path: input.into(),
        output_path: output.into(),
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
            x: 10,
            y: 20,
            width: 30,
            height: 40,
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
            start_ns: 100,
            end_ns: 200,
        },
    );
    audit.verification_required_frame_count = 2;

    MaskPlan {
        schema_version: 1,
        capture_id: "capture-receipt-001".to_string(),
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

fn unmatched_snapshot() -> SecureShareObservationSnapshot {
    SecureShareObservationSnapshot {
        frame_size: FrameSize {
            width: 1280,
            height: 720,
        },
        frames: vec![CaptureFrameMetadata {
            frame_index: 0,
            presentation_time_ns: 900,
            frame_status: FrameStatus::Complete,
            content_rect: CaptureRect {
                x: 0,
                y: 0,
                width: 1280,
                height: 720,
            },
            bounding_rect: CaptureRect {
                x: 0,
                y: 0,
                width: 1280,
                height: 720,
            },
            scale_factor: 1.0,
            content_scale: 1.0,
        }],
        accessibility: Vec::new(),
        vision: vec![dropsquash_core::VisionObservation {
            rect: PixelRect {
                x: 80,
                y: 120,
                width: 360,
                height: 28,
            },
            time_range: TimeRangeNs {
                start_ns: 100,
                end_ns: 200,
            },
            kind: dropsquash_core::VisionObservationKind::TextRecognition,
            confidence: Confidence::CERTAIN,
        }],
    }
}

fn verification() -> VerificationExpectations {
    VerificationExpectations {
        no_audio: true,
        strip_metadata: true,
        verification_policy_version: "secure-share-rd-1".to_string(),
    }
}
