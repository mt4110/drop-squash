use dropsquash_core::{AppError, CaptureFrameMetadata, FrameSize, Result};

use super::{raw_sck_frame_info_from_sample_buffer, FrameMetadataProvider, NativeSampleBuffer};

pub trait NativeSampleBufferProvider {
    fn frame_size(&self) -> Result<FrameSize>;

    fn visit_sample_buffers(
        &self,
        visitor: &mut dyn FnMut(&NativeSampleBuffer) -> Result<()>,
    ) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct SampleBufferFrameMetadataProvider<P> {
    provider: P,
}

impl<P> SampleBufferFrameMetadataProvider<P> {
    pub fn new(provider: P) -> Self {
        Self { provider }
    }
}

impl<P> FrameMetadataProvider for SampleBufferFrameMetadataProvider<P>
where
    P: NativeSampleBufferProvider,
{
    fn frame_size(&self) -> Result<FrameSize> {
        self.provider.frame_size()
    }

    fn frames(&self) -> Result<Vec<CaptureFrameMetadata>> {
        let mut frame_index = 0;
        let mut frames = Vec::new();
        self.provider.visit_sample_buffers(&mut |sample_buffer| {
            let frame_info = raw_sck_frame_info_from_sample_buffer(sample_buffer, frame_index)?;
            frames.push(frame_info.into_metadata()?);
            frame_index += 1;
            Ok(())
        })?;
        if frames.is_empty() {
            return Err(AppError::InvalidConfig(
                "Secure Share ScreenCaptureKit stream produced no frame metadata".to_string(),
            ));
        }
        Ok(frames)
    }
}
