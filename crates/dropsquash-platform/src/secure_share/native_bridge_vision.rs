use dropsquash_core::{
    AppError, CaptureFrameMetadata, Confidence, FrameSize, Result, TimeRangeNs,
    VisionNormalizedRect, VisionObservation, VisionObservationKind,
};

use crate::NativeVisionObservation;

pub fn vision_observations_from_native(
    values: Vec<NativeVisionObservation>,
    frames: &[CaptureFrameMetadata],
    size: FrameSize,
) -> Result<Vec<VisionObservation>> {
    values
        .into_iter()
        .map(|value| observation(value, frames, size))
        .collect()
}

fn observation(
    value: NativeVisionObservation,
    frames: &[CaptureFrameMetadata],
    size: FrameSize,
) -> Result<VisionObservation> {
    let frame = frames
        .get(value.frame_index as usize)
        .ok_or_else(|| invalid("references a frame that was not retained"))?;
    let kind = match value.kind {
        1 => VisionObservationKind::TextRecognition,
        2 => VisionObservationKind::TextRectangle,
        _ => return Err(invalid("has an unsupported Vision observation kind")),
    };
    VisionNormalizedRect {
        x: value.x.into(),
        y: value.y.into(),
        width: value.width.into(),
        height: value.height.into(),
    }
    .to_observation(
        size,
        TimeRangeNs {
            start_ns: frame.presentation_time_ns,
            end_ns: frame.presentation_time_ns.saturating_add(1),
        },
        kind,
        Confidence {
            detection: value.confidence,
            transform: 1.0,
            policy: 1.0,
        },
    )
    .ok_or_else(|| invalid("has invalid normalized geometry"))
}

fn invalid(reason: &str) -> AppError {
    AppError::InvalidConfig(format!("Secure Share native Vision observation {reason}"))
}

#[cfg(test)]
#[path = "native_bridge_vision/tests.rs"]
mod tests;
