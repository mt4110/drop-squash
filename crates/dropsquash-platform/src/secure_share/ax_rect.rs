use dropsquash_core::{
    AppError, AxObservation, AxObservationKind, Confidence, PixelRect, Result, TimeRangeNs,
};
use objc2_core_foundation::{CGPoint, CGSize};

pub fn observation_from_point_size(
    point: CGPoint,
    size: CGSize,
    time_range: TimeRangeNs,
) -> Result<AxObservation> {
    Ok(AxObservation {
        rect: pixel_rect(point, size)?,
        time_range,
        kind: AxObservationKind::UnknownClientArea,
        confidence: Confidence::CERTAIN,
    })
}

fn pixel_rect(point: CGPoint, size: CGSize) -> Result<PixelRect> {
    Ok(PixelRect {
        x: non_negative(point.x, "x")?,
        y: non_negative(point.y, "y")?,
        width: positive(size.width, "width")?,
        height: positive(size.height, "height")?,
    })
}

fn non_negative(value: f64, name: &str) -> Result<u32> {
    if !value.is_finite() || value < 0.0 || value > u32::MAX as f64 {
        return Err(invalid(&format!("AX rect {name} is invalid")));
    }
    Ok(value.round() as u32)
}

fn positive(value: f64, name: &str) -> Result<u32> {
    if value <= 0.0 {
        return Err(invalid(&format!("AX rect {name} is empty")));
    }
    non_negative(value, name)
}

fn invalid(reason: &str) -> AppError {
    AppError::InvalidConfig(format!("Secure Share Accessibility observation {reason}"))
}
