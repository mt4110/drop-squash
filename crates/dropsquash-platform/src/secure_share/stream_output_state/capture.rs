use dropsquash_core::{FrameSize, PixelRect, TimeRangeNs};
use std::time::Duration;

use super::super::NativeSampleBuffer;
use super::super::RawSckFrameStatus;
use super::{RawSckFrameInfo, SckStreamOutputState};

impl SckStreamOutputState {
    pub(crate) fn vision_input(&mut self) -> (FrameSize, bool) {
        let needs_vision = requires_vision(self.frame_size, &self.fixed_mask_rects)
            && self.admitted_vision_frames < self.vision_frame_limit;
        self.admitted_vision_frames += usize::from(needs_vision);
        (self.frame_size, needs_vision)
    }

    pub(crate) fn capture_admitted_sample_buffer(
        &mut self,
        sample: &NativeSampleBuffer,
        frame: RawSckFrameInfo,
        vision: dropsquash_core::Result<Vec<dropsquash_core::VisionObservation>>,
        vision_elapsed: Option<Duration>,
    ) {
        if self.last_written_time_ns >= Some(frame.presentation_time_ns) {
            self.reject_frame(
                "Secure Share Vision completed frames out of presentation order".into(),
            );
            return;
        }
        if let Some(elapsed) = vision_elapsed {
            self.live_mask.record_vision(elapsed);
        }
        let result = vision.and_then(|mut vision| {
            let mut regions = self.fixed_mask_rects.clone();
            regions.extend(vision.iter().map(|item| item.rect));
            let proof = match self.recording.append(sample, &regions)? {
                Some(proof) => proof,
                None => super::super::live_mask::blacken_regions(sample, &regions)?,
            };
            self.live_mask.record(proof);
            self.frame_infos.push(frame);
            self.vision.append(&mut vision);
            self.last_written_time_ns = Some(frame.presentation_time_ns);
            Ok(())
        });
        if let Err(error) = result {
            self.reject_frame(error.to_string());
        }
    }

    pub(in super::super) fn reject_noncomplete_frame(&mut self, status: RawSckFrameStatus) {
        self.reject_frame(format!(
            "Secure Share observed non-complete ScreenCaptureKit frame status: {status:?}"
        ));
    }

    pub(in super::super) fn accept_frame_continuity(&mut self, frame: RawSckFrameInfo) -> bool {
        if let Err(error) = self.continuity.accept(frame) {
            self.reject_frame(format!("Secure Share observed {error}"));
            return false;
        }
        true
    }

    pub(super) fn reject_frame(&mut self, error: String) {
        self.recording.discard();
        self.errors.push(error);
    }
}

pub(crate) fn requires_vision(frame_size: FrameSize, fixed_masks: &[PixelRect]) -> bool {
    !fixed_masks.iter().any(|rect| {
        rect.x == 0
            && rect.y == 0
            && rect.width == frame_size.width
            && rect.height == frame_size.height
    })
}

pub(crate) fn frame_time_range(start_ns: u64) -> TimeRangeNs {
    TimeRangeNs {
        start_ns,
        end_ns: start_ns.saturating_add(1),
    }
}

#[cfg(test)]
mod tests {
    use super::requires_vision;
    use dropsquash_core::{FrameSize, PixelRect};

    #[test]
    fn full_frame_mask_skips_capture_time_vision() {
        let size = FrameSize {
            width: 640,
            height: 480,
        };
        assert!(!requires_vision(
            size,
            &[PixelRect {
                x: 0,
                y: 0,
                width: 640,
                height: 480,
            }]
        ));
    }
}
