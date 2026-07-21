use dropsquash_core::{AppError, AxObservation, FrameSize, PixelRect, Result};

use super::SckCaptureTarget;

pub(super) fn map_to_capture(
    observations: Vec<AxObservation>,
    target: &SckCaptureTarget,
    frame_size: FrameSize,
) -> Result<Vec<AxObservation>> {
    let scale_x = scale(frame_size.width, target.frame.width, "x")?;
    let scale_y = scale(frame_size.height, target.frame.height, "y")?;
    observations
        .into_iter()
        .filter(|item| inside_target(item.rect, target))
        .map(|item| {
            Ok(AxObservation {
                rect: map_rect(item.rect, target, frame_size, scale_x, scale_y)?,
                ..item
            })
        })
        .collect()
}

fn inside_target(rect: PixelRect, target: &SckCaptureTarget) -> bool {
    let left = i64::from(target.frame.x);
    let top = i64::from(target.frame.y);
    let right = left + i64::from(target.frame.width);
    let bottom = top + i64::from(target.frame.height);
    let rect_left = i64::from(rect.x);
    let rect_top = i64::from(rect.y);
    rect_left >= left
        && rect_top >= top
        && rect_left + i64::from(rect.width) <= right
        && rect_top + i64::from(rect.height) <= bottom
}

fn map_rect(
    rect: PixelRect,
    target: &SckCaptureTarget,
    frame_size: FrameSize,
    scale_x: f64,
    scale_y: f64,
) -> Result<PixelRect> {
    let x = f64::from(rect.x) - f64::from(target.frame.x);
    let y = f64::from(rect.y) - f64::from(target.frame.y);
    let right = x + f64::from(rect.width);
    let bottom = y + f64::from(rect.height);
    debug_assert!(x >= 0.0 && y >= 0.0);
    debug_assert!(right <= f64::from(target.frame.width));
    debug_assert!(bottom <= f64::from(target.frame.height));
    let width = length(f64::from(rect.width) * scale_x, frame_size.width, "width")?;
    let height = length(
        f64::from(rect.height) * scale_y,
        frame_size.height,
        "height",
    )?;
    Ok(PixelRect {
        x: pixel(x * scale_x, frame_size.width, "x")?,
        y: pixel(y * scale_y, frame_size.height, "y")?,
        width,
        height,
    })
}

fn scale(output: u32, input: u32, axis: &str) -> Result<f64> {
    (input > 0)
        .then(|| f64::from(output) / f64::from(input))
        .ok_or_else(|| invalid(&format!("capture {axis} scale is invalid")))
}

fn pixel(value: f64, bound: u32, axis: &str) -> Result<u32> {
    if !value.is_finite() || value < 0.0 || value > f64::from(bound) {
        return Err(invalid(&format!(
            "Accessibility {axis} conversion is invalid"
        )));
    }
    Ok(value.round() as u32)
}

fn length(value: f64, bound: u32, axis: &str) -> Result<u32> {
    let value = pixel(value, bound, axis)?;
    (value > 0)
        .then_some(value)
        .ok_or_else(|| invalid(&format!("Accessibility {axis} conversion is empty")))
}

fn invalid(reason: &str) -> AppError {
    AppError::InvalidConfig(format!("Secure Share {reason}"))
}
