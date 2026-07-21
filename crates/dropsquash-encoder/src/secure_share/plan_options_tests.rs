use dropsquash_core::{
    Confidence, FrameMaskPlan, FrameSize, FrameStatus, MaskMode, MaskPlan, MaskPlanAudit,
    MaskPolicy, MaskReason, MaskRegion, ObservationSource, PixelRect, RegionPolicy,
    SecureShareOptions, VerificationExpectations,
};

use super::options_for_frame;

mod strict;

#[test]
fn plan_options_require_a_matching_frame_and_size() {
    let options = SecureShareOptions {
        mask_mode: MaskMode::SolidBlack,
        mask_rects: Vec::new(),
        mask_plan: Some(plan()),
    };
    let resolved = options_for_frame(
        &options,
        0,
        FrameSize {
            width: 4,
            height: 4,
        },
    )
    .unwrap();

    assert_eq!(resolved.mask_rects.len(), 1);
    assert!(options_for_frame(
        &options,
        1,
        FrameSize {
            width: 4,
            height: 4
        }
    )
    .is_err());
    assert!(options_for_frame(
        &options,
        0,
        FrameSize {
            width: 8,
            height: 4
        }
    )
    .is_err());
}

#[test]
fn plan_options_reject_noise_for_verified_export() {
    let options = SecureShareOptions {
        mask_mode: MaskMode::BlackNoise,
        mask_rects: Vec::new(),
        mask_plan: Some(plan()),
    };

    assert!(options_for_frame(
        &options,
        0,
        FrameSize {
            width: 4,
            height: 4,
        },
    )
    .is_err());
}

pub(super) fn plan() -> MaskPlan {
    MaskPlan {
        schema_version: 1,
        capture_id: "test-plan".to_string(),
        frame_size: FrameSize {
            width: 4,
            height: 4,
        },
        frames: vec![FrameMaskPlan {
            frame_index: 0,
            presentation_time_ns: 0,
            frame_status: FrameStatus::Complete,
            regions: vec![MaskRegion {
                rect: PixelRect {
                    x: 0,
                    y: 0,
                    width: 4,
                    height: 4,
                },
                policy: RegionPolicy::Sensitive,
                reason: MaskReason::VisionText,
                sources: vec![ObservationSource::VisionTextRecognition],
                confidence: Confidence::CERTAIN,
                expansion_px: 0,
            }],
        }],
        policy: MaskPolicy::StrictReveal,
        audit: MaskPlanAudit::clean(),
        verification_expectations: VerificationExpectations {
            no_audio: true,
            strip_metadata: true,
            verification_policy_version: "test".to_string(),
        },
    }
}
