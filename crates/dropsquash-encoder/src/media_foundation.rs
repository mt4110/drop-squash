use async_trait::async_trait;
use dropsquash_core::{AppError, EncodeJob, EncodeResult, Result};

use crate::{EncoderBackend, EncoderCapabilities};

#[derive(Debug, Clone, Default)]
pub struct MediaFoundationEncoder;

#[async_trait]
impl EncoderBackend for MediaFoundationEncoder {
    fn name(&self) -> &'static str {
        "media-foundation"
    }

    fn probe_capabilities(&self) -> Result<EncoderCapabilities> {
        Ok(EncoderCapabilities {
            backend_name: self.name().to_string(),
            available: false,
            hardware_acceleration: false,
            supports_h264: false,
            supports_hevc: false,
            supports_metadata_strip: false,
            input_extensions: Vec::new(),
        })
    }

    async fn encode(&self, _job: EncodeJob) -> Result<EncodeResult> {
        Err(AppError::Encoder(format!(
            "native-encoder-unavailable: {} is not implemented yet; no external media fallback is used",
            self.name()
        )))
    }
}
