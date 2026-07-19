use dropsquash_core::{AppError, CaptureRect, Result};

pub fn capture_rect(rect: objc2_foundation::NSRect) -> Result<CaptureRect> {
    Ok(CaptureRect {
        x: i32_from(rect.origin.x, "rect x")?,
        y: i32_from(rect.origin.y, "rect y")?,
        width: u32_from(rect.size.width, "rect width")?,
        height: u32_from(rect.size.height, "rect height")?,
    })
}

fn i32_from(value: f64, name: &str) -> Result<i32> {
    if !value.is_finite() || value < i32::MIN as f64 || value > i32::MAX as f64 {
        return Err(AppError::InvalidConfig(format!(
            "Secure Share SCK {name} is invalid"
        )));
    }
    Ok(value.round() as i32)
}

fn u32_from(value: f64, name: &str) -> Result<u32> {
    if !value.is_finite() || value < 0.0 || value > u32::MAX as f64 {
        return Err(AppError::InvalidConfig(format!(
            "Secure Share SCK {name} is invalid"
        )));
    }
    Ok(value.round() as u32)
}
