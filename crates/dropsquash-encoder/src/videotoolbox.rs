use async_trait::async_trait;
use dropsquash_core::{EncodeJob, EncodeResult, Result};
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

use crate::{EncodeProgressReporter, EncoderBackend, EncoderCapabilities};

mod capabilities;
#[cfg(target_os = "macos")]
mod encode;
#[cfg(target_os = "macos")]
mod finalize;
#[cfg(target_os = "macos")]
mod paths;
#[cfg(target_os = "macos")]
mod presets;
#[cfg(target_os = "macos")]
mod session;

#[cfg(target_os = "macos")]
use objc2::rc::autoreleasepool;

#[derive(Debug, Clone, Default)]
pub struct AppleNativeEncoder;

#[async_trait]
impl EncoderBackend for AppleNativeEncoder {
    fn name(&self) -> &'static str {
        "apple-native"
    }

    fn probe_capabilities(&self) -> Result<EncoderCapabilities> {
        Ok(capabilities::probe(self.name()))
    }

    async fn encode(&self, job: EncodeJob) -> Result<EncodeResult> {
        #[cfg(target_os = "macos")]
        {
            return autoreleasepool(|_| {
                encode::encode_with_avfoundation(job, None, CancellationToken::new())
            });
        }

        #[cfg(not(target_os = "macos"))]
        Err(dropsquash_core::AppError::Encoder(
            "Apple native backend is only implemented on macOS in Phase 0".to_string(),
        ))
    }

    async fn encode_with_progress(
        &self,
        job: EncodeJob,
        reporter: Arc<dyn EncodeProgressReporter>,
    ) -> Result<EncodeResult> {
        #[cfg(target_os = "macos")]
        {
            return self
                .encode_with_progress_and_cancel(job, reporter, CancellationToken::new())
                .await;
        }

        #[cfg(not(target_os = "macos"))]
        {
            reporter.report(0.0);
            let result = self.encode(job).await;
            if result.is_ok() {
                reporter.report(1.0);
            }
            result
        }
    }

    async fn encode_with_progress_and_cancel(
        &self,
        job: EncodeJob,
        reporter: Arc<dyn EncodeProgressReporter>,
        cancel: CancellationToken,
    ) -> Result<EncodeResult> {
        #[cfg(target_os = "macos")]
        {
            return autoreleasepool(|_| {
                encode::encode_with_avfoundation(job, Some(reporter.as_ref()), cancel)
            });
        }

        #[cfg(not(target_os = "macos"))]
        {
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
}

#[cfg(all(test, target_os = "macos"))]
mod tests;
