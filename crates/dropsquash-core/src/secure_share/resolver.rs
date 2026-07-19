use super::{
    Confidence, FrameMaskPlan, FrameSize, MaskPlanAudit, MaskReason, MaskRegion, ObservationSource,
    PixelRect, RegionPolicy, TimeRangeNs,
};

pub(super) fn assign_observation(
    frames: &mut [FrameMaskPlan],
    audit: &mut MaskPlanAudit,
    frame_size: FrameSize,
    strict_reveal: bool,
    time_range: TimeRangeNs,
    region: MaskRegion,
) {
    if push_region_in_range(frames, time_range, region.clone()) || !strict_reveal {
        return;
    }
    if let Some(source) = region.sources.first().copied() {
        audit.record_unmatched(source, region.reason, time_range);
    }
    push_full_frame_verification_required(frames, frame_size);
}

pub(super) fn count_verification_required_frames(frames: &[FrameMaskPlan]) -> usize {
    frames
        .iter()
        .filter(|frame| {
            frame
                .regions
                .iter()
                .any(|region| region.reason == MaskReason::VerificationRequired)
        })
        .count()
}

fn push_region_in_range(
    frames: &mut [FrameMaskPlan],
    time_range: TimeRangeNs,
    region: MaskRegion,
) -> bool {
    let mut matched = false;
    for frame in frames
        .iter_mut()
        .filter(|frame| time_range.contains(frame.presentation_time_ns))
    {
        frame.regions.push(region.clone());
        matched = true;
    }
    matched
}

fn push_full_frame_verification_required(frames: &mut [FrameMaskPlan], frame_size: FrameSize) {
    for frame in frames {
        frame.regions.push(MaskRegion {
            rect: PixelRect {
                x: 0,
                y: 0,
                width: frame_size.width,
                height: frame_size.height,
            },
            policy: RegionPolicy::Unknown,
            reason: MaskReason::VerificationRequired,
            sources: vec![ObservationSource::PolicyDefault],
            confidence: Confidence {
                detection: 0.0,
                transform: 0.0,
                policy: 1.0,
            },
            expansion_px: 0,
        });
    }
}
