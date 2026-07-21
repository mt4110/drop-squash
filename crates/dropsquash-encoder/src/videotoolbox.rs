use async_trait::async_trait;
use dropsquash_core::{EncodeJob, EncodeResult, Result};
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

use crate::{EncodeProgressReporter, EncoderBackend, EncoderCapabilities};

#[cfg(target_os = "macos")]
mod auto_detect;
mod capabilities;
#[cfg(target_os = "macos")]
mod encode;
#[cfg(target_os = "macos")]
mod finalize;
#[cfg(target_os = "macos")]
mod mask;
#[cfg(target_os = "macos")]
mod paths;
#[cfg(target_os = "macos")]
mod pixel_buffer;
#[cfg(target_os = "macos")]
mod presets;
#[cfg(target_os = "macos")]
mod secure_share;
#[cfg(target_os = "macos")]
mod secure_share_flow;
#[cfg(target_os = "macos")]
mod secure_share_settings;
#[cfg(target_os = "macos")]
mod secure_share_track_policy;
#[cfg(target_os = "macos")]
mod secure_share_verify;
#[cfg(target_os = "macos")]
mod session;
#[cfg(not(target_os = "macos"))]
mod unavailable;

#[cfg(target_os = "macos")]
pub(crate) use auto_detect::detect_auto_mask_rects;
#[cfg(target_os = "macos")]
pub(crate) use mask::apply_destructive_mask_rgba_for_proof;
#[cfg(target_os = "macos")]
use objc2::rc::autoreleasepool;
#[cfg(target_os = "macos")]
pub(crate) use secure_share_verify::verify_experimental_masked_output;
#[cfg(target_os = "macos")]
pub(crate) use secure_share_verify::verify_masked_output;

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
        unavailable::encode(job).await
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
        unavailable::encode_with_progress(job, reporter).await
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
        unavailable::encode_with_progress_and_cancel(job, reporter, cancel).await
    }
}

#[cfg(all(test, target_os = "macos"))]
mod tests;
