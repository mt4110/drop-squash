use dropsquash_core::{
    AppError, CaptureFrameMetadata, FrameSize, PixelRect, Result, VisionObservation,
};

use super::frame_continuity::FrameContinuity;
use super::{RawSckFrameInfo, SckLiveMaskEvidence};
use std::path::PathBuf;

mod admission;
mod capture;
mod recording;

pub(crate) use capture::frame_time_range;

#[derive(Debug)]
pub struct SckStreamOutputState {
    frame_size: FrameSize,
    frame_infos: Vec<RawSckFrameInfo>,
    continuity: FrameContinuity,
    vision: Vec<VisionObservation>,
    fixed_mask_rects: Vec<PixelRect>,
    live_mask: SckLiveMaskEvidence,
    recording: recording::RecordingState,
    errors: Vec<String>,
    next_frame_index: u64,
    last_written_time_ns: Option<u64>,
    vision_frame_limit: usize,
    admitted_vision_frames: usize,
}

impl SckStreamOutputState {
    pub fn new(frame_size: FrameSize) -> Self {
        Self {
            frame_size,
            frame_infos: Vec::new(),
            continuity: FrameContinuity::default(),
            vision: Vec::new(),
            fixed_mask_rects: Vec::new(),
            live_mask: SckLiveMaskEvidence::default(),
            recording: recording::RecordingState::disabled(),
            errors: Vec::new(),
            next_frame_index: 0,
            last_written_time_ns: None,
            vision_frame_limit: usize::MAX,
            admitted_vision_frames: 0,
        }
    }

    pub fn with_recording(
        frame_size: FrameSize,
        output_path: PathBuf,
        fixed_mask_rects: Vec<PixelRect>,
        vision_frame_limit: usize,
    ) -> Self {
        let mut state = Self::new(frame_size);
        state.recording = recording::RecordingState::enabled(output_path);
        state.fixed_mask_rects = fixed_mask_rects;
        state.vision_frame_limit = vision_frame_limit;
        state
    }

    pub fn frame_size(&self) -> FrameSize {
        self.frame_size
    }

    pub fn frames(&self) -> Result<Vec<CaptureFrameMetadata>> {
        if let Some(error) = self.errors.first() {
            return Err(AppError::InvalidConfig(format!(
                "Secure Share ScreenCaptureKit stream output rejected frame metadata: {error}{}",
                self.live_mask.vision_summary()
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
                "Secure Share ScreenCaptureKit stream output rejected Vision observation: {error}{}",
                self.live_mask.vision_summary()
            )));
        }
        Ok(self.vision.clone())
    }

    pub fn live_mask_evidence(&self) -> Result<SckLiveMaskEvidence> {
        self.vision_observations()?;
        Ok(self.live_mask)
    }

    pub fn finish_recording(&mut self) -> Result<Option<PathBuf>> {
        if let Some(error) = self.errors.first() {
            return Err(AppError::InvalidConfig(error.clone()));
        }
        self.recording.finish()
    }

    pub fn discard_recording(&mut self) {
        self.recording.discard();
    }
}
