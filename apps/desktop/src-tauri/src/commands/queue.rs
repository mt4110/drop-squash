use std::path::PathBuf;

use dropsquash_core::EncodeJob;
use dropsquash_queue::{QueueEvent, QueueItem, QueueJobId};

use super::dto::ConvertRequest;
use crate::state::AppState;

pub fn enqueue(
    app_state: tauri::State<'_, AppState>,
    request: ConvertRequest,
) -> Result<QueueEvent, String> {
    app_state.enqueue_job(job_from_request(request))
}

pub fn start_next(app_state: tauri::State<'_, AppState>) -> Result<Option<QueueEvent>, String> {
    app_state.start_next_job()
}

pub fn cancel_pending(
    app_state: tauri::State<'_, AppState>,
    id: u64,
) -> Result<Option<QueueEvent>, String> {
    app_state.cancel_queued_job(QueueJobId(id))
}

pub fn block_pending(
    app_state: tauri::State<'_, AppState>,
    error: String,
) -> Result<Vec<QueueEvent>, String> {
    app_state.block_queued_jobs(error)
}

pub fn clear_completed(app_state: tauri::State<'_, AppState>) -> Result<Vec<QueueItem>, String> {
    app_state.clear_completed_jobs()
}

fn job_from_request(request: ConvertRequest) -> EncodeJob {
    EncodeJob {
        input_path: PathBuf::from(request.input_path),
        output_dir: PathBuf::from(request.output_dir),
        profile: request.profile,
        output_size: request.output_size,
        source_policy: request.source_policy,
    }
}

#[cfg(test)]
mod tests;
