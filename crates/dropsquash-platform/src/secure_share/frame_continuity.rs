use dropsquash_core::CaptureRect;

use super::RawSckFrameInfo;

const CAPTURE_FRAMES_PER_SECOND: u64 = 30;
pub(super) const MAX_FRAME_GAP_NS: u64 = 5 * (1_000_000_000 / CAPTURE_FRAMES_PER_SECOND);

#[derive(Debug, Default)]
pub(super) struct FrameContinuity {
    last_time_ns: Option<u64>,
    geometry: Option<FrameGeometry>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FrameGeometry {
    content_rect: CaptureRect,
    bounding_rect: CaptureRect,
    scale_factor_bits: u32,
    content_scale_bits: u32,
}

impl FrameContinuity {
    pub(super) fn accept(&mut self, frame: RawSckFrameInfo) -> Result<(), String> {
        if self.last_time_ns >= Some(frame.presentation_time_ns) {
            return Err("non-monotonic ScreenCaptureKit presentation time".to_string());
        }
        if let Some(last) = self.last_time_ns {
            let gap = frame.presentation_time_ns - last;
            if gap > MAX_FRAME_GAP_NS {
                return Err(format!(
                    "ScreenCaptureKit presentation time gap {gap}ns exceeded the {MAX_FRAME_GAP_NS}ns capture policy"
                ));
            }
        }
        let geometry = FrameGeometry::from(frame);
        if self.geometry.is_some_and(|expected| expected != geometry) {
            return Err(
                "ScreenCaptureKit frame geometry or scale changed during capture".to_string(),
            );
        }
        self.last_time_ns = Some(frame.presentation_time_ns);
        self.geometry = Some(geometry);
        Ok(())
    }
}

impl From<RawSckFrameInfo> for FrameGeometry {
    fn from(frame: RawSckFrameInfo) -> Self {
        Self {
            content_rect: frame.content_rect,
            bounding_rect: frame.bounding_rect,
            scale_factor_bits: frame.scale_factor.to_bits(),
            content_scale_bits: frame.content_scale.to_bits(),
        }
    }
}
