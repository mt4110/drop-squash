use std::path::PathBuf;

use dropsquash_core::PixelRect;
use dropsquash_core::Result;

use super::super::recording_writer::SckRecordingWriter;
use super::super::NativeSampleBuffer;

#[derive(Debug)]
pub(super) struct RecordingState {
    path: Option<PathBuf>,
    writer: Option<SckRecordingWriter>,
}

impl RecordingState {
    pub(super) fn disabled() -> Self {
        Self {
            path: None,
            writer: None,
        }
    }

    pub(super) fn enabled(path: PathBuf) -> Self {
        Self {
            path: Some(path),
            writer: None,
        }
    }

    pub(super) fn append(
        &mut self,
        sample: &NativeSampleBuffer,
        regions: &[PixelRect],
    ) -> Result<Option<super::super::live_mask::FrameMaskProof>> {
        let Some(path) = &self.path else {
            return Ok(None);
        };
        if self.writer.is_none() {
            self.writer = Some(SckRecordingWriter::start(path.clone(), sample)?);
        }
        self.writer
            .as_ref()
            .unwrap()
            .append(sample, regions)
            .map(Some)
    }

    pub(super) fn finish(&mut self) -> Result<Option<PathBuf>> {
        self.writer
            .take()
            .map(SckRecordingWriter::finish)
            .transpose()
    }

    pub(super) fn discard(&mut self) {
        if let Some(writer) = self.writer.take() {
            writer.discard();
        }
        if let Some(path) = self.path.take() {
            let _ = std::fs::remove_file(path);
        }
    }
}

#[cfg(test)]
#[path = "recording_tests.rs"]
mod tests;
