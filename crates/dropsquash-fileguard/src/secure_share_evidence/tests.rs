use std::fs;

use dropsquash_core::{
    required_capture_continuity_watches, strict_shield_exposure_coverage, Confidence,
    ExposureCoverageStatus, FrameMaskPlan, FrameSize, FrameStatus, MaskPlan, MaskPlanAudit,
    MaskPolicy, MaskReason, MaskRegion, ObservationSource, PixelRect, RegionPolicy,
    VerificationExpectations,
};

use super::{
    canonical_payload, verify_secure_share_evidence, SecureShareEvidence, SignedSecureShareEvidence,
};
use crate::{sha256_hex_for_file, sign_evidence};

#[test]
fn verifies_a_signed_sidecar_bound_to_its_video() {
    let directory = tempfile::tempdir().unwrap();
    let video = directory.path().join("clip.mp4");
    let sidecar = directory.path().join("clip.mask-plan.json");
    fs::write(&video, b"video").unwrap();
    write_sidecar(&sidecar, &video, directory.path().join("key"));

    let evidence = verify_secure_share_evidence(&video, &sidecar).unwrap();

    assert_eq!(evidence.output_name, "clip.mp4");
}

#[test]
fn rejects_a_video_replaced_after_its_sidecar_was_signed() {
    let directory = tempfile::tempdir().unwrap();
    let video = directory.path().join("clip.mp4");
    let sidecar = directory.path().join("clip.mask-plan.json");
    fs::write(&video, b"original video").unwrap();
    write_sidecar(&sidecar, &video, directory.path().join("key"));
    fs::write(&video, b"replacement video").unwrap();

    let error = verify_secure_share_evidence(&video, &sidecar)
        .expect_err("replaced video must not match signed evidence");
    assert!(
        error.to_string().contains("output SHA-256 does not match"),
        "unexpected replacement error: {error}"
    );
}

#[test]
fn rejects_a_sidecar_with_raw_private_observation_fields() {
    let directory = tempfile::tempdir().unwrap();
    let video = directory.path().join("clip.mp4");
    let sidecar = directory.path().join("clip.mask-plan.json");
    fs::write(&video, b"video").unwrap();
    write_sidecar(&sidecar, &video, directory.path().join("key"));
    let mut value: serde_json::Value =
        serde_json::from_slice(&fs::read(&sidecar).unwrap()).unwrap();
    value["evidence"]["plan"]["vision"] = serde_json::json!([{"text": "secret"}]);
    fs::write(&sidecar, serde_json::to_vec(&value).unwrap()).unwrap();

    assert!(verify_secure_share_evidence(&video, &sidecar).is_err());
}

#[test]
fn rejects_a_sidecar_with_a_raw_focused_input_value() {
    let directory = tempfile::tempdir().unwrap();
    let video = directory.path().join("clip.mp4");
    let sidecar = directory.path().join("clip.mask-plan.json");
    fs::write(&video, b"video").unwrap();
    write_sidecar(&sidecar, &video, directory.path().join("key"));
    let mut value: serde_json::Value =
        serde_json::from_slice(&fs::read(&sidecar).unwrap()).unwrap();
    value["evidence"]["plan"]["audit"]["focusedValue"] = serde_json::json!("secret");
    fs::write(&sidecar, serde_json::to_vec(&value).unwrap()).unwrap();

    assert!(verify_secure_share_evidence(&video, &sidecar).is_err());
}

#[test]
fn rejects_a_sidecar_with_an_unknown_audit_field() {
    let directory = tempfile::tempdir().unwrap();
    let video = directory.path().join("clip.mp4");
    let sidecar = directory.path().join("clip.mask-plan.json");
    fs::write(&video, b"video").unwrap();
    write_sidecar(&sidecar, &video, directory.path().join("key"));
    let mut value: serde_json::Value =
        serde_json::from_slice(&fs::read(&sidecar).unwrap()).unwrap();
    value["evidence"]["plan"]["audit"]["unexpected"] = serde_json::json!(true);
    fs::write(&sidecar, serde_json::to_vec(&value).unwrap()).unwrap();

    assert!(verify_secure_share_evidence(&video, &sidecar).is_err());
}

#[test]
fn rejects_a_validly_signed_legacy_schema() {
    let directory = tempfile::tempdir().unwrap();
    let video = directory.path().join("clip.mp4");
    let sidecar = directory.path().join("clip.mask-plan.json");
    fs::write(&video, b"video").unwrap();
    let mut evidence = evidence_for(&video);
    evidence.schema_version = 1;
    let signature = sign_evidence(
        &canonical_payload(&evidence).unwrap(),
        &directory.path().join("key"),
    )
    .unwrap();
    let signed = SignedSecureShareEvidence {
        evidence,
        signature,
    };
    fs::write(&sidecar, serde_json::to_vec(&signed).unwrap()).unwrap();

    let error = verify_secure_share_evidence(&video, &sidecar)
        .unwrap_err()
        .to_string();
    assert!(error.contains("legacy or unsupported schema"));
}

#[test]
fn rejects_current_plan_schema_without_continuity_attestation() {
    let directory = tempfile::tempdir().unwrap();
    let video = directory.path().join("clip.mp4");
    let sidecar = directory.path().join("clip.mask-plan.json");
    fs::write(&video, b"video").unwrap();
    let mut evidence = evidence_for(&video);
    evidence.plan.schema_version = 2;
    evidence.plan.audit.capture_continuity_attested = None;
    let signature = sign_evidence(
        &canonical_payload(&evidence).unwrap(),
        &directory.path().join("key"),
    )
    .unwrap();
    fs::write(
        &sidecar,
        serde_json::to_vec(&SignedSecureShareEvidence {
            evidence,
            signature,
        })
        .unwrap(),
    )
    .unwrap();

    let error = verify_secure_share_evidence(&video, &sidecar).unwrap_err();
    assert!(error.to_string().contains("continuity attestation"));
}

#[test]
fn rejects_native_bridge_evidence_without_native_capture_boundary() {
    let directory = tempfile::tempdir().unwrap();
    let video = directory.path().join("clip.mp4");
    let sidecar = directory.path().join("clip.mask-plan.json");
    fs::write(&video, b"video").unwrap();
    let mut evidence = evidence_for(&video);
    evidence
        .plan
        .verification_expectations
        .verification_policy_version = "phase5-native-bridge-v1".into();
    evidence.plan.audit.capture_backend = None;
    write_signed(&sidecar, evidence, directory.path().join("key"));

    let error = verify_secure_share_evidence(&video, &sidecar).unwrap_err();
    assert!(error.to_string().contains("Apple-native capture boundary"));
}

#[test]
fn rejects_native_bridge_evidence_without_frame_destruction_counts() {
    let directory = tempfile::tempdir().unwrap();
    let video = directory.path().join("clip.mp4");
    let sidecar = directory.path().join("clip.mask-plan.json");
    fs::write(&video, b"video").unwrap();
    let mut evidence = evidence_for(&video);
    evidence
        .plan
        .verification_expectations
        .verification_policy_version = "phase5-native-bridge-v1".into();
    evidence.plan.audit.native_verified_frame_count = 0;
    write_signed(&sidecar, evidence, directory.path().join("key"));

    let error = verify_secure_share_evidence(&video, &sidecar).unwrap_err();
    assert!(error.to_string().contains("per-frame native destruction"));
}

#[test]
fn rejects_a_validly_signed_legacy_mask_plan_schema() {
    let directory = tempfile::tempdir().unwrap();
    let video = directory.path().join("clip.mp4");
    let sidecar = directory.path().join("clip.mask-plan.json");
    fs::write(&video, b"video").unwrap();
    let mut evidence = evidence_for(&video);
    evidence.plan.schema_version = 1;
    write_signed(&sidecar, evidence, directory.path().join("key"));

    let error = verify_secure_share_evidence(&video, &sidecar).unwrap_err();
    assert!(error.to_string().contains("legacy or unsupported"));
}

#[test]
fn rejects_current_plan_schema_without_continuity_watch_evidence() {
    let directory = tempfile::tempdir().unwrap();
    let video = directory.path().join("clip.mp4");
    let sidecar = directory.path().join("clip.mask-plan.json");
    fs::write(&video, b"video").unwrap();
    let mut evidence = evidence_for(&video);
    evidence.plan.schema_version = 2;
    evidence.plan.audit.capture_continuity_attested = Some(true);
    evidence.plan.audit.capture_continuity_watches.clear();
    write_signed(&sidecar, evidence, directory.path().join("key"));

    let error = verify_secure_share_evidence(&video, &sidecar).unwrap_err();
    assert!(error.to_string().contains("continuity watch evidence"));
}

#[test]
fn rejects_current_plan_schema_without_exposure_coverage() {
    let directory = tempfile::tempdir().unwrap();
    let video = directory.path().join("clip.mp4");
    let sidecar = directory.path().join("clip.mask-plan.json");
    fs::write(&video, b"video").unwrap();
    let mut evidence = evidence_for(&video);
    evidence.plan.audit.exposure_coverage.clear();
    write_signed(&sidecar, evidence, directory.path().join("key"));

    let error = verify_secure_share_evidence(&video, &sidecar).unwrap_err();
    assert!(error.to_string().contains("exposure coverage"));
}

#[test]
fn rejects_current_plan_schema_with_weakened_exposure_coverage() {
    let directory = tempfile::tempdir().unwrap();
    let video = directory.path().join("clip.mp4");
    let sidecar = directory.path().join("clip.mask-plan.json");
    fs::write(&video, b"video").unwrap();
    let mut evidence = evidence_for(&video);
    evidence.plan.audit.exposure_coverage[0].status = ExposureCoverageStatus::DetectedNotCovered;
    write_signed(&sidecar, evidence, directory.path().join("key"));

    let error = verify_secure_share_evidence(&video, &sidecar).unwrap_err();
    assert!(error.to_string().contains("exposure coverage"));
}

#[test]
fn rejects_duplicate_exposure_coverage_paths() {
    let directory = tempfile::tempdir().unwrap();
    let video = directory.path().join("clip.mp4");
    let sidecar = directory.path().join("clip.mask-plan.json");
    fs::write(&video, b"video").unwrap();
    let mut evidence = evidence_for(&video);
    evidence
        .plan
        .audit
        .exposure_coverage
        .push(evidence.plan.audit.exposure_coverage[0].clone());
    write_signed(&sidecar, evidence, directory.path().join("key"));

    let error = verify_secure_share_evidence(&video, &sidecar).unwrap_err();
    assert!(error.to_string().contains("exposure coverage"));
}

#[test]
fn rejects_a_validly_signed_strict_plan_with_observation_geometry() {
    let directory = tempfile::tempdir().unwrap();
    let video = directory.path().join("clip.mp4");
    let sidecar = directory.path().join("clip.mask-plan.json");
    fs::write(&video, b"video").unwrap();
    let mut evidence = evidence_for(&video);
    evidence.plan.frames[0].regions.push(MaskRegion {
        rect: PixelRect {
            x: 0,
            y: 0,
            width: 1,
            height: 1,
        },
        policy: RegionPolicy::Sensitive,
        reason: MaskReason::VisionText,
        sources: vec![ObservationSource::VisionTextRecognition],
        confidence: Confidence::CERTAIN,
        expansion_px: 0,
    });
    let signature = sign_evidence(
        &canonical_payload(&evidence).unwrap(),
        &directory.path().join("key"),
    )
    .unwrap();
    fs::write(
        &sidecar,
        serde_json::to_vec(&SignedSecureShareEvidence {
            evidence,
            signature,
        })
        .unwrap(),
    )
    .unwrap();

    assert!(verify_secure_share_evidence(&video, &sidecar).is_err());
}

#[test]
fn rejects_a_validly_signed_smart_mask_plan() {
    let directory = tempfile::tempdir().unwrap();
    let video = directory.path().join("clip.mp4");
    let sidecar = directory.path().join("clip.mask-plan.json");
    fs::write(&video, b"video").unwrap();
    let mut evidence = evidence_for(&video);
    evidence.plan.policy = MaskPolicy::SmartMask;
    let signature = sign_evidence(
        &canonical_payload(&evidence).unwrap(),
        &directory.path().join("key"),
    )
    .unwrap();
    fs::write(
        &sidecar,
        serde_json::to_vec(&SignedSecureShareEvidence {
            evidence,
            signature,
        })
        .unwrap(),
    )
    .unwrap();

    let error = verify_secure_share_evidence(&video, &sidecar).unwrap_err();
    assert!(error.to_string().contains("Strict Shield"));
}

#[test]
fn rejects_a_validly_signed_private_capture_identifier() {
    let directory = tempfile::tempdir().unwrap();
    let video = directory.path().join("clip.mp4");
    let sidecar = directory.path().join("clip.mask-plan.json");
    fs::write(&video, b"video").unwrap();
    let mut evidence = evidence_for(&video);
    evidence.plan.capture_id = "customer-hanako@example.test".into();
    write_signed(&sidecar, evidence, directory.path().join("key"));

    let error = verify_secure_share_evidence(&video, &sidecar).unwrap_err();

    assert!(error
        .to_string()
        .contains("non-redacted Strict Shield capture identifier"));
}

#[test]
fn rejects_a_validly_signed_strict_plan_without_canonical_unknown_region() {
    let directory = tempfile::tempdir().unwrap();
    let video = directory.path().join("clip.mp4");
    let sidecar = directory.path().join("clip.mask-plan.json");
    fs::write(&video, b"video").unwrap();
    let mut evidence = evidence_for(&video);
    evidence.plan.frames[0].regions[0].policy = RegionPolicy::Sensitive;
    let signature = sign_evidence(
        &canonical_payload(&evidence).unwrap(),
        &directory.path().join("key"),
    )
    .unwrap();
    fs::write(
        &sidecar,
        serde_json::to_vec(&SignedSecureShareEvidence {
            evidence,
            signature,
        })
        .unwrap(),
    )
    .unwrap();

    assert!(verify_secure_share_evidence(&video, &sidecar).is_err());
}

#[test]
fn rejects_a_validly_signed_empty_strict_plan() {
    let directory = tempfile::tempdir().unwrap();
    let video = directory.path().join("clip.mp4");
    let sidecar = directory.path().join("clip.mask-plan.json");
    fs::write(&video, b"video").unwrap();
    let mut evidence = evidence_for(&video);
    evidence.plan.frames.clear();
    let signature = sign_evidence(
        &canonical_payload(&evidence).unwrap(),
        &directory.path().join("key"),
    )
    .unwrap();
    fs::write(
        &sidecar,
        serde_json::to_vec(&SignedSecureShareEvidence {
            evidence,
            signature,
        })
        .unwrap(),
    )
    .unwrap();

    assert!(verify_secure_share_evidence(&video, &sidecar).is_err());
}

#[test]
fn rejects_a_validly_signed_strict_plan_with_repeated_frame_index() {
    let directory = tempfile::tempdir().unwrap();
    let video = directory.path().join("clip.mp4");
    let sidecar = directory.path().join("clip.mask-plan.json");
    fs::write(&video, b"video").unwrap();
    let mut evidence = evidence_for(&video);
    evidence.plan.frames.push(evidence.plan.frames[0].clone());
    let signature = sign_evidence(
        &canonical_payload(&evidence).unwrap(),
        &directory.path().join("key"),
    )
    .unwrap();
    fs::write(
        &sidecar,
        serde_json::to_vec(&SignedSecureShareEvidence {
            evidence,
            signature,
        })
        .unwrap(),
    )
    .unwrap();

    assert!(verify_secure_share_evidence(&video, &sidecar).is_err());
}

#[test]
fn rejects_a_validly_signed_strict_plan_that_allows_metadata() {
    let directory = tempfile::tempdir().unwrap();
    let video = directory.path().join("clip.mp4");
    let sidecar = directory.path().join("clip.mask-plan.json");
    fs::write(&video, b"video").unwrap();
    let mut evidence = evidence_for(&video);
    evidence.plan.verification_expectations.strip_metadata = false;
    let signature = sign_evidence(
        &canonical_payload(&evidence).unwrap(),
        &directory.path().join("key"),
    )
    .unwrap();
    fs::write(
        &sidecar,
        serde_json::to_vec(&SignedSecureShareEvidence {
            evidence,
            signature,
        })
        .unwrap(),
    )
    .unwrap();

    assert!(verify_secure_share_evidence(&video, &sidecar).is_err());
}

#[test]
fn rejects_a_validly_signed_unknown_strict_plan_schema() {
    let directory = tempfile::tempdir().unwrap();
    let video = directory.path().join("clip.mp4");
    let sidecar = directory.path().join("clip.mask-plan.json");
    fs::write(&video, b"video").unwrap();
    let mut evidence = evidence_for(&video);
    evidence.plan.schema_version = 99;
    let signature = sign_evidence(
        &canonical_payload(&evidence).unwrap(),
        &directory.path().join("key"),
    )
    .unwrap();
    fs::write(
        &sidecar,
        serde_json::to_vec(&SignedSecureShareEvidence {
            evidence,
            signature,
        })
        .unwrap(),
    )
    .unwrap();

    assert!(verify_secure_share_evidence(&video, &sidecar).is_err());
}

#[test]
fn rejects_a_validly_signed_strict_plan_with_empty_frame_size() {
    let directory = tempfile::tempdir().unwrap();
    let video = directory.path().join("clip.mp4");
    let sidecar = directory.path().join("clip.mask-plan.json");
    fs::write(&video, b"video").unwrap();
    let mut evidence = evidence_for(&video);
    evidence.plan.frame_size.width = 0;
    write_signed(&sidecar, evidence, directory.path().join("key"));

    assert!(verify_secure_share_evidence(&video, &sidecar).is_err());
}

#[test]
fn rejects_a_validly_signed_strict_plan_with_uncertain_region() {
    let directory = tempfile::tempdir().unwrap();
    let video = directory.path().join("clip.mp4");
    let sidecar = directory.path().join("clip.mask-plan.json");
    fs::write(&video, b"video").unwrap();
    let mut evidence = evidence_for(&video);
    evidence.plan.frames[0].regions[0].confidence = Confidence {
        detection: 0.9,
        transform: 1.0,
        policy: 1.0,
    };
    write_signed(&sidecar, evidence, directory.path().join("key"));

    assert!(verify_secure_share_evidence(&video, &sidecar).is_err());
}

fn write_sidecar(sidecar: &std::path::Path, video: &std::path::Path, key: std::path::PathBuf) {
    write_signed(sidecar, evidence_for(video), key);
}

fn write_signed(sidecar: &std::path::Path, evidence: SecureShareEvidence, key: std::path::PathBuf) {
    let signature = sign_evidence(&canonical_payload(&evidence).unwrap(), &key).unwrap();
    let signed = SignedSecureShareEvidence {
        evidence,
        signature,
    };
    fs::write(sidecar, serde_json::to_vec(&signed).unwrap()).unwrap();
}

fn evidence_for(video: &std::path::Path) -> SecureShareEvidence {
    SecureShareEvidence {
        schema_version: 2,
        output_name: "clip.mp4".into(),
        output_sha256: sha256_hex_for_file(video).unwrap(),
        plan: plan(),
    }
}

fn plan() -> MaskPlan {
    let mut audit = MaskPlanAudit::clean();
    audit.capture_continuity_attested = Some(true);
    audit.capture_backend = Some("apple_native_capture_v1".into());
    audit.record_native_destruction(1, 1, 1);
    audit.capture_continuity_watches = required_capture_continuity_watches()
        .iter()
        .map(|watch| (*watch).to_string())
        .collect();
    audit.exposure_coverage = strict_shield_exposure_coverage();
    MaskPlan {
        schema_version: 2,
        capture_id: "secure-share-recording-local".into(),
        frame_size: FrameSize {
            width: 1,
            height: 1,
        },
        frames: vec![FrameMaskPlan {
            frame_index: 0,
            presentation_time_ns: 0,
            frame_status: FrameStatus::Complete,
            regions: vec![MaskRegion {
                rect: PixelRect {
                    x: 0,
                    y: 0,
                    width: 1,
                    height: 1,
                },
                policy: RegionPolicy::Unknown,
                reason: MaskReason::UnknownRegion,
                sources: vec![ObservationSource::ScreenCaptureKitFrame],
                confidence: Confidence::CERTAIN,
                expansion_px: 0,
            }],
        }],
        policy: MaskPolicy::StrictReveal,
        audit,
        verification_expectations: VerificationExpectations {
            no_audio: true,
            strip_metadata: true,
            verification_policy_version: "test".into(),
        },
    }
}
