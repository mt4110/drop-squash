use super::super::{raw_sck_frame_info_from_sample_buffer, NativeSampleBuffer, RawSckFrameStatus};
use super::{RawSckFrameInfo, SckStreamOutputState};

impl SckStreamOutputState {
    pub(crate) fn reject_analysis_capacity(&mut self) {
        self.reject_frame("Secure Share local Vision analysis capacity was exceeded".into());
    }

    pub(crate) fn admit_sample_buffer(
        &mut self,
        sample_buffer: &NativeSampleBuffer,
    ) -> Option<RawSckFrameInfo> {
        match raw_sck_frame_info_from_sample_buffer(sample_buffer, self.next_frame_index) {
            Ok(frame) if frame.status == RawSckFrameStatus::Complete => self.accept(frame),
            Ok(frame) => {
                self.reject_noncomplete_frame(frame.status);
                None
            }
            Err(error) => {
                self.reject_frame(error.to_string());
                None
            }
        }
    }

    fn accept(&mut self, frame: RawSckFrameInfo) -> Option<RawSckFrameInfo> {
        self.accept_frame_continuity(frame).then(|| {
            self.next_frame_index += 1;
            frame
        })
    }
}
