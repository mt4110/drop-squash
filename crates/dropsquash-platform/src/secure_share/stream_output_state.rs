use dropsquash_core::{
    AppError, CaptureFrameMetadata, FrameSize, Result, TimeRangeNs, VisionObservation,
};

use super::{
    raw_sck_frame_info_from_sample_buffer, NativeSampleBuffer, RawSckFrameInfo, SckLiveMaskEvidence,
};

#[derive(Debug)]
pub struct SckStreamOutputState {
    frame_size: FrameSize,
    frame_infos: Vec<RawSckFrameInfo>,
    vision: Vec<VisionObservation>,
    live_mask: SckLiveMaskEvidence,
    errors: Vec<String>,
}

impl SckStreamOutputState {
    pub fn new(frame_size: FrameSize) -> Self {
        Self {
            frame_size,
            frame_infos: Vec::new(),
            vision: Vec::new(),
            live_mask: SckLiveMaskEvidence::default(),
            errors: Vec::new(),
        }
    }

    pub fn frame_size(&self) -> FrameSize {
        self.frame_size
    }

    pub fn capture_sample_buffer(&mut self, sample_buffer: &NativeSampleBuffer) {
        match raw_sck_frame_info_from_sample_buffer(sample_buffer, self.frame_infos.len() as u64) {
            Ok(frame_info) => self.capture_accepted_sample_buffer(sample_buffer, frame_info),
            Err(error) => self.errors.push(error.to_string()),
        }
    }

    fn capture_accepted_sample_buffer(
        &mut self,
        sample_buffer: &NativeSampleBuffer,
        frame_info: RawSckFrameInfo,
    ) {
        match super::native_vision_observe::observe_sample_buffer(
            sample_buffer,
            self.frame_size,
            frame_time_range(frame_info.presentation_time_ns),
        ) {
            Ok(mut vision) => {
                match super::live_mask::blacken_observed_text(sample_buffer, &vision) {
                    Ok(proof) => {
                        self.live_mask.record(proof);
                        self.frame_infos.push(frame_info);
                        self.vision.append(&mut vision);
                    }
                    Err(error) => self.errors.push(error.to_string()),
                }
            }
            Err(error) => self.errors.push(error.to_string()),
        }
    }

    pub fn frames(&self) -> Result<Vec<CaptureFrameMetadata>> {
        if let Some(error) = self.errors.first() {
            return Err(AppError::InvalidConfig(format!(
                "Secure Share ScreenCaptureKit stream output rejected frame metadata: {error}"
            )));
        }
        if self.frame_infos.is_empty() {
            return Err(AppError::InvalidConfig(
                "Secure Share ScreenCaptureKit stream output produced no frame metadata"
                    .to_string(),
            ));
        }
        self.frame_infos
            .iter()
            .copied()
            .map(RawSckFrameInfo::into_metadata)
            .collect()
    }

    pub fn vision_observations(&self) -> Result<Vec<VisionObservation>> {
        if let Some(error) = self.errors.first() {
            return Err(AppError::InvalidConfig(format!(
                "Secure Share ScreenCaptureKit stream output rejected Vision observation: {error}"
            )));
        }
        Ok(self.vision.clone())
    }

    pub fn live_mask_evidence(&self) -> Result<SckLiveMaskEvidence> {
        self.vision_observations()?;
        Ok(self.live_mask)
    }
}

fn frame_time_range(start_ns: u64) -> TimeRangeNs {
    TimeRangeNs {
        start_ns,
        end_ns: start_ns.saturating_add(1),
    }
}
