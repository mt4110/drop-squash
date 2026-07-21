use dropsquash_core::{AppError, CaptureFrameMetadata, CaptureRect, FrameStatus, Result};

use crate::NativeFrameMetadata;

pub fn capture_frame_metadata_from_native(
    value: NativeFrameMetadata,
) -> Result<CaptureFrameMetadata> {
    Ok(CaptureFrameMetadata {
        frame_index: value.frame_index,
        presentation_time_ns: super::mach_clock::nanoseconds(value.display_time_ticks)?,
        frame_status: status(value.frame_status)?,
        content_rect: rect(
            value.content_x,
            value.content_y,
            value.content_width,
            value.content_height,
        )?,
        bounding_rect: rect(
            value.bounding_x,
            value.bounding_y,
            value.bounding_width,
            value.bounding_height,
        )?,
        scale_factor: scale(value.scale_factor)?,
        content_scale: scale(value.content_scale)?,
    })
}

fn status(value: i32) -> Result<FrameStatus> {
    match value {
        0 => Ok(FrameStatus::Complete),
        1 => Ok(FrameStatus::Idle),
        2 => Ok(FrameStatus::Blank),
        3 => Ok(FrameStatus::Suspended),
        4 => Ok(FrameStatus::Started),
        5 => Ok(FrameStatus::Stopped),
        _ => Err(invalid("frame status")),
    }
}

fn rect(x: i32, y: i32, width: u32, height: u32) -> Result<CaptureRect> {
    (width > 0 && height > 0)
        .then_some(CaptureRect {
            x,
            y,
            width,
            height,
        })
        .ok_or_else(|| invalid("frame rectangle"))
}

fn scale(value: f32) -> Result<f32> {
    (value.is_finite() && value > 0.0)
        .then_some(value)
        .ok_or_else(|| invalid("frame scale"))
}

fn invalid(field: &str) -> AppError {
    AppError::InvalidConfig(format!("Secure Share native bridge has invalid {field}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_complete_native_metadata() {
        let frame = capture_frame_metadata_from_native(NativeFrameMetadata {
            frame_index: 4,
            display_time_ticks: 0,
            frame_status: 0,
            scale_factor: 2.0,
            content_scale: 1.0,
            content_x: 2,
            content_y: 3,
            content_width: 100,
            content_height: 80,
            bounding_x: 1,
            bounding_y: 2,
            bounding_width: 101,
            bounding_height: 82,
        })
        .unwrap();

        assert_eq!(frame.frame_index, 4);
        assert_eq!(frame.presentation_time_ns, 0);
        assert_eq!(frame.frame_status, FrameStatus::Complete);
        assert_eq!(frame.content_rect.width, 100);
        assert_eq!(frame.bounding_rect.height, 82);
    }

    #[test]
    fn rejects_unknown_native_status() {
        assert!(status(6).is_err());
    }
    #[test]
    fn rejects_empty_rectangles() {
        assert!(rect(0, 0, 0, 1).is_err());
    }
}
