use serde::{Deserialize, Serialize};

use super::{
    Confidence, FrameSize, PixelRect, TimeRangeNs, VisionObservation, VisionObservationKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisionNormalizedRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl VisionNormalizedRect {
    pub fn to_pixel_rect(self, frame_size: FrameSize) -> Option<PixelRect> {
        if !self.valid() || frame_size.width == 0 || frame_size.height == 0 {
            return None;
        }
        let left = self.x;
        let right = self.x + self.width;
        let top = 1.0 - (self.y + self.height);
        let bottom = 1.0 - self.y;
        pixel_rect(frame_size, left, top, right, bottom)
    }

    pub fn to_observation(
        self,
        frame_size: FrameSize,
        time_range: TimeRangeNs,
        kind: VisionObservationKind,
        confidence: Confidence,
    ) -> Option<VisionObservation> {
        Some(VisionObservation {
            rect: self.to_pixel_rect(frame_size)?,
            time_range,
            kind,
            confidence,
        })
    }

    fn valid(self) -> bool {
        self.x.is_finite()
            && self.y.is_finite()
            && self.width.is_finite()
            && self.height.is_finite()
            && self.width > 0.0
            && self.height > 0.0
    }
}

fn pixel_rect(
    frame_size: FrameSize,
    left: f64,
    top: f64,
    right: f64,
    bottom: f64,
) -> Option<PixelRect> {
    let left = clamp(left, 0.0, 1.0);
    let top = clamp(top, 0.0, 1.0);
    let right = clamp(right, 0.0, 1.0);
    let bottom = clamp(bottom, 0.0, 1.0);
    if right <= left || bottom <= top {
        return None;
    }
    let x = floor_pixel(left * frame_size.width as f64);
    let y = floor_pixel(top * frame_size.height as f64);
    let x2 = ceil_pixel(right * frame_size.width as f64);
    let y2 = ceil_pixel(bottom * frame_size.height as f64);
    Some(PixelRect {
        x: x as u32,
        y: y as u32,
        width: (x2 - x) as u32,
        height: (y2 - y) as u32,
    })
}

fn clamp(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}

fn ceil_pixel(value: f64) -> f64 {
    (value - 1e-9).ceil()
}

fn floor_pixel(value: f64) -> f64 {
    (value + 1e-9).floor()
}
