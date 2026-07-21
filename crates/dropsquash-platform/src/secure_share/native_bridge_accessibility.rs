use dropsquash_core::{
    AppError, AxObservation, AxObservationKind, CaptureFrameMetadata, Confidence, FrameSize,
    PixelRect, Result,
};

use super::SckCaptureTarget;
use crate::apple_capture_bridge::NativeAccessibilityObservation;

pub fn accessibility_observations_from_native(
    values: Vec<NativeAccessibilityObservation>,
    target: &SckCaptureTarget,
    frame_size: FrameSize,
    frames: &[CaptureFrameMetadata],
) -> Result<Vec<AxObservation>> {
    let observations = values
        .into_iter()
        .map(|value| native_observation(value, target, frame_size, frames))
        .collect::<Result<Vec<_>>>()?;
    Ok(observations)
}

fn native_observation(
    value: NativeAccessibilityObservation,
    target: &SckCaptureTarget,
    frame_size: FrameSize,
    frames: &[CaptureFrameMetadata],
) -> Result<AxObservation> {
    let kind = match value.kind {
        1 => AxObservationKind::TextElement,
        2 => AxObservationKind::FocusedTextElement,
        _ => return Err(invalid("native Accessibility observation kind is unknown")),
    };
    if value.width == 0 || value.height == 0 {
        return Err(invalid("native Accessibility observation is invalid"));
    }
    Ok(AxObservation {
        rect: map_rect(value, target, frame_size)?,
        time_range: time_range(value.frame_index, frames)?,
        kind,
        confidence: Confidence::CERTAIN,
    })
}

fn time_range(
    frame_index: u64,
    frames: &[CaptureFrameMetadata],
) -> Result<dropsquash_core::TimeRangeNs> {
    let frame = frames
        .get(frame_index as usize)
        .ok_or_else(|| invalid("native Accessibility observation references a missing frame"))?;
    Ok(dropsquash_core::TimeRangeNs {
        start_ns: frame.presentation_time_ns,
        end_ns: frame.presentation_time_ns.saturating_add(1),
    })
}

fn map_rect(
    value: NativeAccessibilityObservation,
    target: &SckCaptureTarget,
    frame_size: FrameSize,
) -> Result<PixelRect> {
    let x = i64::from(value.x) - i64::from(target.frame.x);
    let y = i64::from(value.y) - i64::from(target.frame.y);
    let right = x + i64::from(value.width);
    let bottom = y + i64::from(value.height);
    if x < 0
        || y < 0
        || right > i64::from(target.frame.width)
        || bottom > i64::from(target.frame.height)
    {
        return Err(invalid(
            "native Accessibility observation is outside the target",
        ));
    }
    let scale_x = f64::from(frame_size.width) / f64::from(target.frame.width);
    let scale_y = f64::from(frame_size.height) / f64::from(target.frame.height);
    let width = scaled(i64::from(value.width), scale_x, frame_size.width)?;
    let height = scaled(i64::from(value.height), scale_y, frame_size.height)?;
    if width == 0 || height == 0 {
        return Err(invalid("native Accessibility scale conversion is empty"));
    }
    Ok(PixelRect {
        x: scaled(x, scale_x, frame_size.width)?,
        y: scaled(y, scale_y, frame_size.height)?,
        width,
        height,
    })
}

fn scaled(value: i64, scale: f64, bound: u32) -> Result<u32> {
    let value = (value as f64 * scale).round();
    if !value.is_finite() || value < 0.0 || value > f64::from(bound) {
        return Err(invalid("native Accessibility scale conversion is invalid"));
    }
    Ok(value as u32)
}

fn invalid(reason: &str) -> AppError {
    AppError::InvalidConfig(format!("Secure Share {reason}"))
}

#[cfg(test)]
#[path = "native_bridge_accessibility/tests.rs"]
mod tests;
