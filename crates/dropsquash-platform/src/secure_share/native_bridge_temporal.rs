use dropsquash_core::{
    AppError, CaptureFrameMetadata, Confidence, FrameSize, Result, TemporalObservation,
    TimeRangeNs, VisionNormalizedRect,
};

use crate::NativeTemporalObservation;

pub fn temporal_observations_from_native(
    values: Vec<NativeTemporalObservation>,
    frames: &[CaptureFrameMetadata],
    size: FrameSize,
) -> Result<Vec<TemporalObservation>> {
    values
        .into_iter()
        .map(|value| observation(value, frames, size))
        .collect()
}

fn observation(
    value: NativeTemporalObservation,
    frames: &[CaptureFrameMetadata],
    size: FrameSize,
) -> Result<TemporalObservation> {
    let frame = frames
        .get(value.frame_index as usize)
        .ok_or_else(|| invalid("references a frame that was not retained"))?;
    let rect = VisionNormalizedRect {
        x: value.x.into(),
        y: value.y.into(),
        width: value.width.into(),
        height: value.height.into(),
    }
    .to_pixel_rect(size)
    .ok_or_else(|| invalid("has invalid normalized geometry"))?;
    Ok(TemporalObservation {
        rect,
        time_range: TimeRangeNs {
            start_ns: frame.presentation_time_ns,
            end_ns: frame.presentation_time_ns.saturating_add(1),
        },
        confidence: Confidence::CERTAIN,
    })
}

fn invalid(reason: &str) -> AppError {
    AppError::InvalidConfig(format!("Secure Share native temporal observation {reason}"))
}

#[cfg(test)]
#[path = "native_bridge_temporal/tests.rs"]
mod tests;
