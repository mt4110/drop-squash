use dropsquash_core::{AppError, Result};
use objc2_av_foundation::{AVAssetExportSession, AVAssetExportSessionStatus};

pub(super) fn ensure_completed(export_session: &AVAssetExportSession) -> Result<()> {
    let status = unsafe { export_session.status() };
    if status == AVAssetExportSessionStatus::Completed {
        return Ok(());
    }
    let message = unsafe { export_session.error() }
        .map(|error| error.localizedDescription().to_string())
        .unwrap_or_else(|| format!("Apple export ended with status {}", status.0));
    Err(AppError::Encoder(message))
}
