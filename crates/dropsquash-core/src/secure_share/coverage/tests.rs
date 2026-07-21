use super::super::super::{Confidence, ObservationSource};
use super::super::{
    destructive_coverage, selective_mask_precision, FrameMaskPlan, FrameSize, FrameStatus,
    MaskPlan, MaskPlanAudit, MaskPolicy, MaskReason, MaskRegion, PixelRect, RegionPolicy,
    VerificationExpectations,
};

#[test]
fn calculates_union_without_double_counting_overlaps() {
    let plan = plan(vec![rect(0, 0, 50, 50), rect(25, 0, 50, 50)]);
    let coverage = destructive_coverage(&plan);
    assert_eq!(coverage.max_frame_ppm, 375_000);
    assert_eq!(coverage.mean_frame_ppm, 375_000);
}

#[test]
fn clips_regions_to_the_output_frame() {
    let coverage = destructive_coverage(&plan(vec![rect(90, 90, 20, 20)]));
    assert_eq!(coverage.max_frame_ppm, 10_000);
}

#[test]
fn reports_leaks_and_overmasking_against_fixture_truth() {
    let plan = plan(vec![rect(0, 0, 50, 50)]);
    let precision = selective_mask_precision(
        &plan,
        &[
            PixelRect {
                x: 0,
                y: 0,
                width: 25,
                height: 50,
            },
            PixelRect {
                x: 50,
                y: 0,
                width: 25,
                height: 50,
            },
        ],
    );

    assert_eq!(precision.min_truth_covered_ppm, 500_000);
    assert_eq!(precision.max_mask_outside_truth_ppm, 125_000);
}

#[test]
fn precision_uses_the_expanded_encoder_mask() {
    let mut plan = plan(vec![rect(10, 10, 10, 10)]);
    plan.frames[0].regions[0].expansion_px = 5;
    let precision = selective_mask_precision(&plan, &[rect(5, 5, 20, 20).rect]);
    assert_eq!(precision.min_truth_covered_ppm, 1_000_000);
}

fn plan(regions: Vec<MaskRegion>) -> MaskPlan {
    MaskPlan {
        schema_version: 1,
        capture_id: "fixture".into(),
        frame_size: FrameSize {
            width: 100,
            height: 100,
        },
        frames: vec![FrameMaskPlan {
            frame_index: 0,
            presentation_time_ns: 0,
            frame_status: FrameStatus::Complete,
            regions,
        }],
        policy: MaskPolicy::SmartMask,
        audit: MaskPlanAudit::clean(),
        verification_expectations: VerificationExpectations {
            no_audio: true,
            strip_metadata: true,
            verification_policy_version: "fixture".into(),
        },
    }
}

fn rect(x: u32, y: u32, width: u32, height: u32) -> MaskRegion {
    MaskRegion {
        rect: PixelRect {
            x,
            y,
            width,
            height,
        },
        policy: RegionPolicy::Sensitive,
        reason: MaskReason::VisionText,
        sources: vec![ObservationSource::VisionTextRecognition],
        confidence: Confidence::CERTAIN,
        expansion_px: 0,
    }
}
