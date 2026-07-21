use serde::{Deserialize, Serialize};

use super::{
    Confidence, FrameMaskPlan, FrameSize, FrameStatus, MaskReason, MaskRegion, ObservationSource,
    PixelRect, RegionPolicy,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptureRect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptureFrameMetadata {
    pub frame_index: u64,
    pub presentation_time_ns: u64,
    pub frame_status: FrameStatus,
    pub content_rect: CaptureRect,
    pub bounding_rect: CaptureRect,
    pub scale_factor: f32,
    pub content_scale: f32,
}

impl CaptureFrameMetadata {
    pub fn to_mask_frame(self, frame_size: FrameSize, strict_reveal: bool) -> FrameMaskPlan {
        let regions = if strict_reveal {
            vec![unknown_frame_region(frame_size)]
        } else {
            Vec::new()
        };
        FrameMaskPlan {
            frame_index: self.frame_index,
            presentation_time_ns: self.presentation_time_ns,
            frame_status: self.frame_status,
            regions,
        }
    }
}

fn unknown_frame_region(frame_size: FrameSize) -> MaskRegion {
    MaskRegion {
        rect: PixelRect {
            x: 0,
            y: 0,
            width: frame_size.width,
            height: frame_size.height,
        },
        policy: RegionPolicy::Unknown,
        reason: MaskReason::UnknownRegion,
        sources: vec![ObservationSource::ScreenCaptureKitFrame],
        confidence: Confidence::CERTAIN,
        expansion_px: 0,
    }
}
