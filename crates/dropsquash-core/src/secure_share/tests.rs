use serde_json::{json, Value};

use super::{
    mask_options_for_frame, AxObservation, AxObservationKind, CaptureFrameMetadata, CaptureRect,
    Confidence, CoordinateSpace, ExposureCoverageStatus, FrameMaskPlan, FrameSize, FrameStatus,
    MaskPlan, MaskPlanAudit, MaskPlanDraft, MaskPolicy, MaskReason, MaskRegion, ObservationSource,
    PixelRect, RegionPolicy, TemporalObservation, TimeRangeNs, VerificationExpectations,
    VisionNormalizedRect, VisionObservation, VisionObservationKind,
};

#[test]
fn mask_plan_uses_stable_wire_names() {
    let plan = sample_plan();

    let value = serde_json::to_value(plan).unwrap();

    assert_eq!(value["schemaVersion"], json!(1));
    assert_eq!(value["policy"], json!("strict_reveal"));
    assert_eq!(value["audit"]["verificationRequiredFrameCount"], json!(0));
    assert_eq!(value["frames"][0]["frameStatus"], json!("complete"));
    assert_eq!(value["frames"][0]["regions"][0]["policy"], json!("unknown"));
    assert_eq!(
        value["frames"][0]["regions"][0]["reason"],
        json!("vision_text")
    );
    assert_eq!(
        value["frames"][0]["regions"][0]["sources"][0],
        json!("vision_text_recognition")
    );
}

#[test]
fn mask_plan_round_trips_without_private_text() {
    let value = serde_json::to_value(sample_plan()).unwrap();
    let plan: MaskPlan = serde_json::from_value(value.clone()).unwrap();
    let round_trip = serde_json::to_value(plan).unwrap();

    assert_eq!(round_trip, value);
    assert_no_key_named_text(&round_trip);
}

#[test]
fn legacy_plan_omits_unattested_continuity() {
    let value = serde_json::to_value(sample_plan()).unwrap();

    assert!(value["audit"].get("captureContinuityAttested").is_none());
    assert!(value["audit"].get("captureContinuityWatches").is_none());
}

#[test]
fn attested_continuity_uses_redacted_watch_names() {
    let mut plan = sample_plan();
    plan.schema_version = 2;
    plan.audit.capture_continuity_attested = Some(true);
    plan.audit.capture_continuity_watches = super::required_capture_continuity_watches()
        .iter()
        .map(|watch| (*watch).to_string())
        .collect();
    let value = serde_json::to_value(plan).unwrap();

    assert_eq!(value["audit"]["captureContinuityAttested"], json!(true));
    assert_eq!(
        value["audit"]["captureContinuityWatches"][0],
        json!("display_configuration")
    );
    assert_no_key_named_text(&value);
}

#[test]
fn strict_shield_exposure_coverage_names_risky_input_paths() {
    let coverage = super::strict_shield_exposure_coverage();

    assert!(coverage.iter().any(|item| item.path == "typed_text"));
    assert!(coverage
        .iter()
        .any(|item| item.path == "ime_composition_candidate"));
    assert!(coverage
        .iter()
        .any(|item| item.path == "audio_track" && item.status == ExposureCoverageStatus::Covered));
    assert!(coverage
        .iter()
        .any(|item| item.path == "target_selection_mistake"
            && item.status == ExposureCoverageStatus::FailClosed));
    assert!(coverage.iter().any(
        |item| item.path == "external_capture_or_endpoint_exfiltration"
            && item.status == ExposureCoverageStatus::OutOfScope
    ));
    assert!(coverage
        .iter()
        .any(|item| item.path == "accessibility_vision_disagreement"
            && item.status == ExposureCoverageStatus::DetectedNotCovered));
}

#[test]
fn destructive_policy_wins_when_regions_merge() {
    assert_eq!(
        RegionPolicy::Safe.merged(RegionPolicy::Unknown),
        RegionPolicy::Unknown
    );
    assert_eq!(
        RegionPolicy::Unknown.merged(RegionPolicy::Sensitive),
        RegionPolicy::Sensitive
    );
}

#[test]
fn strict_reveal_destroys_complete_capture_frame() {
    let frame =
        sample_capture_frame(FrameStatus::Complete).to_mask_frame(sample_frame_size(), true);

    assert_eq!(frame.frame_index, 42);
    assert_eq!(frame.presentation_time_ns, 1_250_000_000);
    assert_eq!(frame.regions.len(), 1);
    assert_eq!(frame.regions[0].reason, MaskReason::UnknownRegion);
    assert_eq!(frame.regions[0].rect, sample_full_frame_rect());
    assert_eq!(frame.regions[0].confidence, Confidence::CERTAIN);
}

#[test]
fn strict_reveal_destroys_untrusted_capture_frame() {
    let frame = sample_capture_frame(FrameStatus::Blank).to_mask_frame(sample_frame_size(), true);

    assert_eq!(frame.regions.len(), 1);
    let region = &frame.regions[0];
    assert_eq!(region.policy, RegionPolicy::Unknown);
    assert_eq!(region.reason, MaskReason::UnknownRegion);
    assert_eq!(
        region.sources,
        vec![ObservationSource::ScreenCaptureKitFrame]
    );
    assert_eq!(region.rect, sample_full_frame_rect());
}

#[test]
fn smart_mask_does_not_destroy_untrusted_frame_by_default() {
    let frame = sample_capture_frame(FrameStatus::Blank).to_mask_frame(sample_frame_size(), false);

    assert!(frame.regions.is_empty());
}

#[test]
fn accessibility_text_observation_becomes_sensitive_region() {
    let region = AxObservation {
        rect: sample_text_rect(),
        time_range: sample_time_range(),
        kind: AxObservationKind::TextElement,
        confidence: Confidence::CERTAIN,
    }
    .to_region();

    assert_eq!(region.policy, RegionPolicy::Sensitive);
    assert_eq!(region.reason, MaskReason::AxTextElement);
    assert_eq!(region.sources, vec![ObservationSource::AccessibilityText]);
    assert_eq!(region.rect, sample_text_rect());
}

#[test]
fn focused_accessibility_text_has_its_own_mask_plan_reason() {
    let region = AxObservation {
        rect: sample_text_rect(),
        time_range: sample_time_range(),
        kind: AxObservationKind::FocusedTextElement,
        confidence: Confidence::CERTAIN,
    }
    .to_region();

    assert_eq!(region.reason, MaskReason::AxFocusedTextElement);
    assert_eq!(
        region.sources,
        vec![ObservationSource::AccessibilityFocusedText]
    );
}

#[test]
fn strict_shield_audits_focused_text_without_retaining_pixels() {
    let plan = MaskPlanDraft {
        capture_id: "capture-focused-001".to_string(),
        frame_size: sample_frame_size(),
        frames: vec![sample_capture_frame(FrameStatus::Complete)],
        accessibility: vec![AxObservation {
            rect: sample_text_rect(),
            time_range: sample_time_range(),
            kind: AxObservationKind::FocusedTextElement,
            confidence: Confidence::CERTAIN,
        }],
        vision: Vec::new(),
        temporal: Vec::new(),
        policy: MaskPolicy::StrictReveal,
        verification_expectations: sample_verification(),
    }
    .into_mask_plan();

    assert_eq!(plan.audit.focused_text_observation_count, 1);
    assert_eq!(plan.frames[0].regions[0].rect, sample_full_frame_rect());
    let value = serde_json::to_value(plan).unwrap();
    assert_eq!(value["audit"]["focusedTextObservationCount"], json!(1));
    assert_no_key_named_text(&value);
}

#[test]
fn accessibility_unknown_client_area_keeps_fail_closed_reason() {
    let region = AxObservation {
        rect: sample_text_rect(),
        time_range: sample_time_range(),
        kind: AxObservationKind::UnknownClientArea,
        confidence: Confidence::CERTAIN,
    }
    .to_region();

    assert_eq!(region.policy, RegionPolicy::Sensitive);
    assert_eq!(region.reason, MaskReason::AxUnknownClientArea);
    assert_eq!(region.sources, vec![ObservationSource::AccessibilityWindow]);
}

#[test]
fn vision_text_shape_does_not_need_private_text_content() {
    let region = VisionObservation {
        rect: sample_text_rect(),
        time_range: sample_time_range(),
        kind: VisionObservationKind::TextRectangle,
        confidence: Confidence {
            detection: 0.74,
            transform: 0.95,
            policy: 1.0,
        },
    }
    .to_region();

    assert_eq!(region.policy, RegionPolicy::Sensitive);
    assert_eq!(region.reason, MaskReason::VisionTextShape);
    assert_eq!(region.sources, vec![ObservationSource::VisionTextRectangle]);
    assert_eq!(region.expansion_px, 8);
}

#[test]
fn vision_normalized_rect_maps_lower_left_to_top_left_pixels() {
    let rect = VisionNormalizedRect {
        x: 0.25,
        y: 0.10,
        width: 0.50,
        height: 0.20,
    }
    .to_pixel_rect(FrameSize {
        width: 800,
        height: 600,
    })
    .unwrap();

    assert_eq!(
        rect,
        PixelRect {
            x: 200,
            y: 420,
            width: 400,
            height: 120,
        }
    );
}

#[test]
fn vision_normalized_rect_clamps_to_frame() {
    let rect = VisionNormalizedRect {
        x: -0.10,
        y: 0.90,
        width: 0.40,
        height: 0.30,
    }
    .to_pixel_rect(sample_frame_size())
    .unwrap();

    assert_eq!(rect.x, 0);
    assert_eq!(rect.y, 0);
    assert_eq!(rect.width, 384);
}

#[test]
fn vision_normalized_rect_builds_text_observation_without_text() {
    let observation = VisionNormalizedRect {
        x: 0.10,
        y: 0.20,
        width: 0.30,
        height: 0.10,
    }
    .to_observation(
        sample_frame_size(),
        sample_time_range(),
        VisionObservationKind::TextRecognition,
        Confidence::CERTAIN,
    )
    .unwrap();

    assert_eq!(observation.kind, VisionObservationKind::TextRecognition);
    assert_eq!(observation.rect.width, 384);
}

#[test]
fn coordinate_space_maps_content_relative_rects_to_output_pixels() {
    let rect = sample_coordinate_space()
        .to_output_rect(CaptureRect {
            x: 110,
            y: 220,
            width: 320,
            height: 180,
        })
        .unwrap();

    assert_eq!(
        rect,
        PixelRect {
            x: 20,
            y: 40,
            width: 640,
            height: 360,
        }
    );
}

#[test]
fn coordinate_space_clamps_out_of_bounds_rects() {
    let rect = sample_coordinate_space()
        .to_output_rect(CaptureRect {
            x: 0,
            y: 0,
            width: 900,
            height: 600,
        })
        .unwrap();

    assert_eq!(rect, sample_full_frame_rect());
}

#[test]
fn coordinate_space_rejects_zero_sized_rects() {
    assert!(sample_coordinate_space()
        .to_output_rect(CaptureRect {
            x: 120,
            y: 240,
            width: 0,
            height: 10,
        })
        .is_none());
}

#[test]
fn mask_plan_draft_combines_capture_ax_and_vision_observations() {
    let plan = MaskPlanDraft {
        capture_id: "capture-combined-001".to_string(),
        frame_size: sample_frame_size(),
        frames: vec![sample_capture_frame(FrameStatus::Complete)],
        accessibility: vec![sample_ax_observation()],
        vision: vec![sample_vision_observation()],
        temporal: Vec::new(),
        policy: MaskPolicy::StrictReveal,
        verification_expectations: sample_verification(),
    }
    .into_mask_plan();

    assert_eq!(plan.schema_version, 1);
    assert_eq!(plan.frames.len(), 1);
    assert_eq!(plan.frames[0].regions.len(), 1);
    assert_eq!(plan.audit.accessibility_observation_count, 1);
    assert_eq!(plan.audit.vision_observation_count, 1);
    assert_eq!(plan.audit.cross_source_overlap_count, 0);
    assert_eq!(plan.audit.accessibility_only_count, 1);
    assert_eq!(plan.audit.vision_only_count, 1);
    assert_eq!(plan.frames[0].regions[0].reason, MaskReason::UnknownRegion);
}

#[test]
fn strict_reveal_draft_keeps_untrusted_frame_mask() {
    let plan = MaskPlanDraft {
        capture_id: "capture-untrusted-001".to_string(),
        frame_size: sample_frame_size(),
        frames: vec![sample_capture_frame(FrameStatus::Idle)],
        accessibility: vec![sample_ax_observation()],
        vision: Vec::new(),
        temporal: Vec::new(),
        policy: MaskPolicy::StrictReveal,
        verification_expectations: sample_verification(),
    }
    .into_mask_plan();

    assert_eq!(plan.frames[0].regions.len(), 1);
    assert_eq!(plan.frames[0].regions[0].reason, MaskReason::UnknownRegion);
    assert_eq!(plan.frames[0].regions[0].rect, sample_full_frame_rect());
}

#[test]
fn mask_plan_draft_assigns_observations_by_frame_time() {
    let plan = MaskPlanDraft {
        capture_id: "capture-timed-001".to_string(),
        frame_size: sample_frame_size(),
        frames: vec![
            sample_capture_frame_at(0, 100_000_000, FrameStatus::Complete),
            sample_capture_frame_at(1, 200_000_000, FrameStatus::Complete),
            sample_capture_frame_at(2, 300_000_000, FrameStatus::Complete),
        ],
        accessibility: vec![AxObservation {
            rect: sample_text_rect(),
            time_range: TimeRangeNs {
                start_ns: 150_000_000,
                end_ns: 250_000_000,
            },
            kind: AxObservationKind::TextElement,
            confidence: Confidence::CERTAIN,
        }],
        vision: Vec::new(),
        temporal: Vec::new(),
        policy: MaskPolicy::SmartMask,
        verification_expectations: sample_verification(),
    }
    .into_mask_plan();

    assert_eq!(plan.frames[0].regions.len(), 0);
    assert_eq!(plan.frames[1].regions.len(), 1);
    assert_eq!(plan.frames[2].regions.len(), 0);
    assert!(plan.frames[1]
        .regions
        .iter()
        .any(|region| region.reason == MaskReason::AxTextElement));
}

#[test]
fn smart_mask_treats_native_pixel_change_as_sensitive() {
    let plan = MaskPlanDraft {
        capture_id: "capture-temporal-001".to_string(),
        frame_size: sample_frame_size(),
        frames: vec![sample_capture_frame(FrameStatus::Complete)],
        accessibility: Vec::new(),
        vision: Vec::new(),
        temporal: vec![TemporalObservation {
            rect: sample_text_rect(),
            time_range: sample_time_range(),
            confidence: Confidence::CERTAIN,
        }],
        policy: MaskPolicy::SmartMask,
        verification_expectations: sample_verification(),
    }
    .into_mask_plan();

    let region = &plan.frames[0].regions[0];
    assert_eq!(region.reason, MaskReason::UnknownRegion);
    assert_eq!(region.sources, vec![ObservationSource::TemporalTracker]);
    assert_eq!(plan.audit.temporal_observation_count, 1);
    let value = serde_json::to_value(plan).unwrap();
    assert_eq!(value["audit"]["temporalObservationCount"], json!(1));
    assert_no_key_named_text(&value);
}

#[test]
fn mask_plan_coalesces_nearby_same_source_regions() {
    let plan = MaskPlanDraft {
        capture_id: "capture-coalesce-001".to_string(),
        frame_size: sample_frame_size(),
        frames: vec![sample_capture_frame(FrameStatus::Complete)],
        accessibility: Vec::new(),
        vision: vec![
            sample_vision_observation_at(PixelRect {
                x: 100,
                y: 200,
                width: 20,
                height: 16,
            }),
            sample_vision_observation_at(PixelRect {
                x: 126,
                y: 202,
                width: 24,
                height: 16,
            }),
        ],
        temporal: Vec::new(),
        policy: MaskPolicy::SmartMask,
        verification_expectations: sample_verification(),
    }
    .into_mask_plan();

    assert_eq!(plan.frames[0].regions.len(), 1);
    let vision = plan.frames[0]
        .regions
        .iter()
        .find(|region| region.reason == MaskReason::VisionText)
        .unwrap();
    assert_eq!(
        vision.rect,
        PixelRect {
            x: 100,
            y: 200,
            width: 50,
            height: 18,
        }
    );
}

#[test]
fn mask_plan_does_not_coalesce_different_sources() {
    let plan = MaskPlanDraft {
        capture_id: "capture-coalesce-002".to_string(),
        frame_size: sample_frame_size(),
        frames: vec![sample_capture_frame(FrameStatus::Complete)],
        accessibility: vec![AxObservation {
            rect: PixelRect {
                x: 100,
                y: 200,
                width: 80,
                height: 20,
            },
            time_range: sample_time_range(),
            kind: AxObservationKind::TextElement,
            confidence: Confidence::CERTAIN,
        }],
        vision: vec![sample_vision_observation_at(PixelRect {
            x: 110,
            y: 202,
            width: 40,
            height: 16,
        })],
        temporal: Vec::new(),
        policy: MaskPolicy::SmartMask,
        verification_expectations: sample_verification(),
    }
    .into_mask_plan();

    assert_eq!(plan.frames[0].regions.len(), 2);
}

#[test]
fn mask_options_for_frame_expands_and_clamps_regions() {
    let plan = MaskPlan {
        schema_version: 1,
        capture_id: "capture-options-001".to_string(),
        frame_size: FrameSize {
            width: 128,
            height: 72,
        },
        frames: vec![FrameMaskPlan {
            frame_index: 7,
            presentation_time_ns: 100,
            frame_status: FrameStatus::Complete,
            regions: vec![MaskRegion {
                rect: PixelRect {
                    x: 2,
                    y: 3,
                    width: 20,
                    height: 10,
                },
                policy: RegionPolicy::Sensitive,
                reason: MaskReason::VisionText,
                sources: vec![ObservationSource::VisionTextRecognition],
                confidence: Confidence::CERTAIN,
                expansion_px: 8,
            }],
        }],
        policy: MaskPolicy::StrictReveal,
        audit: MaskPlanAudit::clean(),
        verification_expectations: sample_verification(),
    };

    let options = mask_options_for_frame(&plan, 7, crate::MaskMode::SolidBlack).unwrap();

    assert_eq!(options.mask_mode, crate::MaskMode::SolidBlack);
    assert_eq!(
        options.mask_rects[0],
        crate::MaskRect {
            x: 0,
            y: 0,
            width: 30,
            height: 21,
        }
    );
}

#[test]
fn strict_reveal_omits_unmatched_observation_geometry() {
    let plan = MaskPlanDraft {
        capture_id: "capture-unmatched-001".to_string(),
        frame_size: sample_frame_size(),
        frames: vec![sample_capture_frame_at(
            0,
            900_000_000,
            FrameStatus::Complete,
        )],
        accessibility: vec![AxObservation {
            rect: sample_text_rect(),
            time_range: TimeRangeNs {
                start_ns: 100_000_000,
                end_ns: 200_000_000,
            },
            kind: AxObservationKind::TextElement,
            confidence: Confidence::CERTAIN,
        }],
        vision: Vec::new(),
        temporal: Vec::new(),
        policy: MaskPolicy::StrictReveal,
        verification_expectations: sample_verification(),
    }
    .into_mask_plan();

    assert_eq!(plan.frames[0].regions.len(), 1);
    assert!(plan.audit.unmatched_observations.is_empty());
    assert_eq!(plan.audit.verification_required_frame_count, 0);
}

fn sample_plan() -> MaskPlan {
    MaskPlan {
        schema_version: 1,
        capture_id: "capture-ja-browser-001".to_string(),
        frame_size: FrameSize {
            width: 1280,
            height: 720,
        },
        frames: vec![FrameMaskPlan {
            frame_index: 12,
            presentation_time_ns: 400_000_000,
            frame_status: FrameStatus::Complete,
            regions: vec![sample_region()],
        }],
        policy: MaskPolicy::StrictReveal,
        audit: MaskPlanAudit::clean(),
        verification_expectations: VerificationExpectations {
            no_audio: true,
            strip_metadata: true,
            verification_policy_version: "secure-share-rd-1".to_string(),
        },
    }
}

fn sample_capture_frame(frame_status: FrameStatus) -> CaptureFrameMetadata {
    sample_capture_frame_at(42, 1_250_000_000, frame_status)
}

fn sample_capture_frame_at(
    frame_index: u64,
    presentation_time_ns: u64,
    frame_status: FrameStatus,
) -> CaptureFrameMetadata {
    CaptureFrameMetadata {
        frame_index,
        presentation_time_ns,
        frame_status,
        content_rect: CaptureRect {
            x: 10,
            y: 20,
            width: 640,
            height: 360,
        },
        bounding_rect: CaptureRect {
            x: 0,
            y: 0,
            width: 1280,
            height: 720,
        },
        scale_factor: 2.0,
        content_scale: 2.0,
    }
}

fn sample_time_range() -> TimeRangeNs {
    TimeRangeNs {
        start_ns: 1_000_000_000,
        end_ns: 1_500_000_000,
    }
}

fn sample_frame_size() -> FrameSize {
    FrameSize {
        width: 1280,
        height: 720,
    }
}

fn sample_full_frame_rect() -> PixelRect {
    PixelRect {
        x: 0,
        y: 0,
        width: 1280,
        height: 720,
    }
}

fn sample_text_rect() -> PixelRect {
    PixelRect {
        x: 80,
        y: 120,
        width: 360,
        height: 28,
    }
}

fn sample_coordinate_space() -> CoordinateSpace {
    CoordinateSpace {
        content_rect: CaptureRect {
            x: 100,
            y: 200,
            width: 640,
            height: 360,
        },
        frame_size: FrameSize {
            width: 1280,
            height: 720,
        },
    }
}

fn sample_region() -> MaskRegion {
    MaskRegion {
        rect: PixelRect {
            x: 100,
            y: 220,
            width: 420,
            height: 32,
        },
        policy: RegionPolicy::Unknown,
        reason: MaskReason::VisionText,
        sources: vec![ObservationSource::VisionTextRecognition],
        confidence: Confidence {
            detection: 0.82,
            transform: 1.0,
            policy: 0.65,
        },
        expansion_px: 8,
    }
}

fn sample_ax_observation() -> AxObservation {
    AxObservation {
        rect: sample_text_rect(),
        time_range: sample_time_range(),
        kind: AxObservationKind::TextElement,
        confidence: Confidence::CERTAIN,
    }
}

fn sample_vision_observation() -> VisionObservation {
    sample_vision_observation_at(PixelRect {
        x: 400,
        y: 300,
        width: 180,
        height: 32,
    })
}

fn sample_vision_observation_at(rect: PixelRect) -> VisionObservation {
    VisionObservation {
        rect,
        time_range: sample_time_range(),
        kind: VisionObservationKind::TextRecognition,
        confidence: Confidence::CERTAIN,
    }
}

fn sample_verification() -> VerificationExpectations {
    VerificationExpectations {
        no_audio: true,
        strip_metadata: true,
        verification_policy_version: "secure-share-rd-1".to_string(),
    }
}

fn assert_no_key_named_text(value: &Value) {
    match value {
        Value::Object(map) => {
            assert!(!map.contains_key("text"));
            for nested in map.values() {
                assert_no_key_named_text(nested);
            }
        }
        Value::Array(items) => {
            for item in items {
                assert_no_key_named_text(item);
            }
        }
        _ => {}
    }
}
