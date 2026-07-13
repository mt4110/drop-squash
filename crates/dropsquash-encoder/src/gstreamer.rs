use async_trait::async_trait;
use dropsquash_core::{AppError, EncodeJob, EncodeResult, Result};

use crate::{EncoderBackend, EncoderCapabilities};

#[derive(Debug, Clone, Default)]
pub struct GStreamerEncoder;

#[async_trait]
impl EncoderBackend for GStreamerEncoder {
    fn name(&self) -> &'static str {
        "gstreamer"
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
        Err(AppError::Encoder(
            "This build cannot use the native encoder on this system.".to_string(),
        ))
    }
}
