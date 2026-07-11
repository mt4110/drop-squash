use async_trait::async_trait;
use dropsquash_core::{EncodeJob, EncodeResult, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncoderCapabilities {
    pub backend_name: String,
    pub available: bool,
    pub hardware_acceleration: bool,
    pub supports_h264: bool,
    pub supports_hevc: bool,
    pub supports_metadata_strip: bool,
    pub input_extensions: Vec<String>,
}

pub trait EncodeProgressReporter: Send + Sync {
    fn report(&self, fraction: f32);
}

#[async_trait]
pub trait EncoderBackend: Send + Sync {
    fn name(&self) -> &'static str;
    fn probe_capabilities(&self) -> Result<EncoderCapabilities>;
    async fn encode(&self, job: EncodeJob) -> Result<EncodeResult>;
    async fn encode_with_progress(
        &self,
        job: EncodeJob,
        reporter: Arc<dyn EncodeProgressReporter>,
    ) -> Result<EncodeResult> {
        self.encode_with_progress_and_cancel(job, reporter, CancellationToken::new())
            .await
    }

    async fn encode_with_progress_and_cancel(
        &self,
        job: EncodeJob,
        reporter: Arc<dyn EncodeProgressReporter>,
        cancel: CancellationToken,
    ) -> Result<EncodeResult> {
        if cancel.is_cancelled() {
            return Err(dropsquash_core::AppError::Cancelled);
        }
        reporter.report(0.0);
        let result = self.encode(job).await;
        if result.is_ok() {
            reporter.report(1.0);
        }
        result
    }
}

#[cfg(test)]
#[derive(Debug, Clone)]
pub struct NoopEncoder {
    pub output_bytes: u64,
}

#[cfg(test)]
#[async_trait]
impl EncoderBackend for NoopEncoder {
    fn name(&self) -> &'static str {
        "noop"
    }

    fn probe_capabilities(&self) -> Result<EncoderCapabilities> {
        Ok(EncoderCapabilities {
            backend_name: self.name().to_string(),
            available: true,
            hardware_acceleration: false,
            supports_h264: true,
            supports_hevc: false,
            supports_metadata_strip: true,
            input_extensions: vec!["mov".to_string(), "mp4".to_string(), "m4v".to_string()],
        })
    }

    async fn encode(&self, job: EncodeJob) -> Result<EncodeResult> {
        Ok(EncodeResult {
            input_path: job.input_path,
            output_path: job.output_dir.join("noop-output.mp4"),
            profile: job.profile,
            original_bytes: 100,
            output_bytes: self.output_bytes,
            success: true,
            error_message: None,
        })
    }
}
