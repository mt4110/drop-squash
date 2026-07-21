use dropsquash_core::{
    AppError, AxObservation, AxObservationKind, CaptureFrameMetadata, Confidence, FrameSize,
    MaskPolicy, PixelRect, Result, TimeRangeNs,
};

use super::SckCaptureTarget;

pub fn observe_accessibility(
    target: &SckCaptureTarget,
    frames: &[CaptureFrameMetadata],
) -> Result<Vec<AxObservation>> {
    let pid = target
        .owner_pid
        .ok_or_else(|| fail_closed("target owner process id is missing"))?;
    super::ax_observation::observe_ax_windows_for_pid(pid, time_range(frames)?)
}

pub fn observe_for_recording(
    target: &SckCaptureTarget,
    frame_size: FrameSize,
    policy: MaskPolicy,
) -> Result<Vec<AxObservation>> {
    let pid = target
        .owner_pid
        .ok_or_else(|| fail_closed("target owner process id is missing"))?;
    let observations = super::ax_observation::observe_ax_windows_for_pid(
        pid,
        TimeRangeNs {
            start_ns: 0,
            end_ns: u64::MAX,
        },
    )?;
    let mut observations = super::ax_capture::map_to_capture(observations, target, frame_size)?;
    if requires_strict_guard(policy) {
        observations.push(strict_guard(frame_size));
    }
    Ok(observations)
}

pub(super) fn requires_strict_guard(policy: MaskPolicy) -> bool {
    policy == MaskPolicy::StrictReveal
}

pub(super) fn strict_guard(frame_size: FrameSize) -> AxObservation {
    AxObservation {
        rect: PixelRect {
            x: 0,
            y: 0,
            width: frame_size.width,
            height: frame_size.height,
        },
        time_range: TimeRangeNs {
            start_ns: 0,
            end_ns: u64::MAX,
        },
        kind: AxObservationKind::UnknownClientArea,
        confidence: Confidence::CERTAIN,
    }
}

fn fail_closed(reason: &str) -> AppError {
    AppError::InvalidConfig(format!(
        "Secure Share Accessibility observation probe {reason}"
    ))
}

pub(super) fn time_range(frames: &[CaptureFrameMetadata]) -> Result<TimeRangeNs> {
    Ok(TimeRangeNs {
        start_ns: frames
            .first()
            .ok_or_else(|| fail_closed("no frame metadata"))?
            .presentation_time_ns,
        end_ns: frames
            .last()
            .ok_or_else(|| fail_closed("no frame metadata"))?
            .presentation_time_ns
            .saturating_add(1),
    })
}
