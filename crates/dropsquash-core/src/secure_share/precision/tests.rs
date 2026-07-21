use super::super::{
    Confidence, FrameMaskPlan, FrameSize, FrameStatus, MaskPlan, MaskPlanAudit, MaskPolicy,
    MaskReason, MaskRegion, ObservationSource, PixelRect, RegionPolicy, VerificationExpectations,
};
use super::selective_mask_precision_for_frames;

#[test]
fn measures_each_frame_against_its_own_truth() {
    let truth = vec![vec![rect(0)], vec![rect(10)]];
    let precision = selective_mask_precision_for_frames(&plan(), &truth).unwrap();
    assert_eq!(precision.min_truth_covered_ppm, 1_000_000);
}

#[test]
fn rejects_a_missing_frame_truth_entry() {
    assert!(selective_mask_precision_for_frames(&plan(), &[vec![rect(0)]]).is_none());
}

fn plan() -> MaskPlan {
    MaskPlan {
        schema_version: 1,
        capture_id: "fixture".into(),
        frame_size: FrameSize {
            width: 20,
            height: 10,
        },
        frames: vec![frame(0), frame(10)],
        policy: MaskPolicy::SmartMask,
        audit: MaskPlanAudit::clean(),
        verification_expectations: VerificationExpectations {
            no_audio: true,
            strip_metadata: true,
            verification_policy_version: "fixture".into(),
        },
    }
}

fn frame(x: u32) -> FrameMaskPlan {
    FrameMaskPlan {
        frame_index: u64::from(x),
        presentation_time_ns: u64::from(x),
        frame_status: FrameStatus::Complete,
        regions: vec![MaskRegion {
            rect: rect(x),
            policy: RegionPolicy::Sensitive,
            reason: MaskReason::VisionText,
            sources: vec![ObservationSource::VisionTextRecognition],
            confidence: Confidence::CERTAIN,
            expansion_px: 0,
        }],
    }
}

fn rect(x: u32) -> PixelRect {
    PixelRect {
        x,
        y: 0,
        width: 10,
        height: 10,
    }
}
