use dropsquash_core::{AppError, EncodeJob, EncodeResult, Result};
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

use crate::EncodeProgressReporter;

pub(super) async fn encode(_job: EncodeJob) -> Result<EncodeResult> {
    Err(AppError::Encoder(
        "Apple native backend is only implemented on macOS in Phase 0".to_string(),
    ))
}

pub(super) async fn encode_with_progress(
    job: EncodeJob,
    reporter: Arc<dyn EncodeProgressReporter>,
) -> Result<EncodeResult> {
    reporter.report(0.0);
    encode(job).await
}

pub(super) async fn encode_with_progress_and_cancel(
    job: EncodeJob,
    reporter: Arc<dyn EncodeProgressReporter>,
    cancel: CancellationToken,
) -> Result<EncodeResult> {
    if cancel.is_cancelled() {
        return Err(AppError::Cancelled);
    }
    encode_with_progress(job, reporter).await
}
