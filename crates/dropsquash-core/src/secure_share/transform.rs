use serde::{Deserialize, Serialize};

use super::{CaptureRect, FrameSize, PixelRect};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoordinateSpace {
    pub content_rect: CaptureRect,
    pub frame_size: FrameSize,
}

impl CoordinateSpace {
    pub fn to_output_rect(self, rect: CaptureRect) -> Option<PixelRect> {
        if self.content_rect.width == 0
            || self.content_rect.height == 0
            || self.frame_size.width == 0
            || self.frame_size.height == 0
            || rect.width == 0
            || rect.height == 0
        {
            return None;
        }
        let x1 = rect.x - self.content_rect.x;
        let y1 = rect.y - self.content_rect.y;
        let x2 = x1 + rect.width as i32;
        let y2 = y1 + rect.height as i32;
        scaled_rect(self, x1, y1, x2, y2)
    }
}

fn scaled_rect(space: CoordinateSpace, x1: i32, y1: i32, x2: i32, y2: i32) -> Option<PixelRect> {
    let sx = space.frame_size.width as f64 / space.content_rect.width as f64;
    let sy = space.frame_size.height as f64 / space.content_rect.height as f64;
    let left = clamp((x1 as f64 * sx).floor(), 0.0, space.frame_size.width as f64);
    let top = clamp(
        (y1 as f64 * sy).floor(),
        0.0,
        space.frame_size.height as f64,
    );
    let right = clamp((x2 as f64 * sx).ceil(), 0.0, space.frame_size.width as f64);
    let bottom = clamp((y2 as f64 * sy).ceil(), 0.0, space.frame_size.height as f64);
    if right <= left || bottom <= top {
        return None;
    }
    Some(PixelRect {
        x: left as u32,
        y: top as u32,
        width: (right - left) as u32,
        height: (bottom - top) as u32,
    })
}

fn clamp(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}
