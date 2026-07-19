use dropsquash_core::{AppError, AxObservation, CaptureFrameMetadata, Result, TimeRangeNs};

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

fn fail_closed(reason: &str) -> AppError {
    AppError::InvalidConfig(format!(
        "Secure Share Accessibility observation probe {reason}"
    ))
}

fn time_range(frames: &[CaptureFrameMetadata]) -> Result<TimeRangeNs> {
    Ok(TimeRangeNs {
        start_ns: frames
            .first()
            .ok_or_else(|| fail_closed("no frame metadata"))?
            .presentation_time_ns,
        end_ns: frames
            .last()
            .ok_or_else(|| fail_closed("no frame metadata"))?
            .presentation_time_ns,
    })
}
