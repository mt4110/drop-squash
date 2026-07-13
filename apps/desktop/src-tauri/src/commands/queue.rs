use std::path::PathBuf;

use dropsquash_core::{EncodeJob, EncodeResult};
use dropsquash_queue::{QueueEvent, QueueItem, QueueJobId};

use super::dto::ConvertRequest;
use crate::state::AppState;

#[tauri::command(rename_all = "camelCase")]
pub fn enqueue_queue_job(
    app_state: tauri::State<'_, AppState>,
    request: ConvertRequest,
) -> Result<QueueEvent, String> {
    app_state.enqueue_job(job_from_request(request))
}

#[tauri::command(rename_all = "camelCase")]
pub fn enqueue_files(
    app_state: tauri::State<'_, AppState>,
    requests: Vec<ConvertRequest>,
) -> Result<Vec<QueueEvent>, String> {
    app_state.enqueue_jobs(requests.into_iter().map(job_from_request).collect())
}

#[tauri::command(rename_all = "camelCase")]
pub fn start_next_queue_job(
    app_state: tauri::State<'_, AppState>,
) -> Result<Option<QueueEvent>, String> {
    app_state.start_next_job()
}

#[tauri::command(rename_all = "camelCase")]
pub fn finish_active_queue_job(
    app_state: tauri::State<'_, AppState>,
    result: EncodeResult,
) -> Result<Option<QueueEvent>, String> {
    app_state.finish_active_queue_job(result)
}

#[tauri::command(rename_all = "camelCase")]
pub fn fail_active_queue_job(
    app_state: tauri::State<'_, AppState>,
    error: String,
) -> Result<Option<QueueEvent>, String> {
    app_state.fail_active_queue_job(error)
}

#[tauri::command(rename_all = "camelCase")]
pub fn cancel_active_queue_job(
    app_state: tauri::State<'_, AppState>,
) -> Result<Option<QueueEvent>, String> {
    app_state.cancel_active_queue_job()
}

#[tauri::command(rename_all = "camelCase")]
pub fn cancel_queued_job(
    app_state: tauri::State<'_, AppState>,
    id: u64,
) -> Result<Option<QueueEvent>, String> {
    app_state.cancel_queued_job(QueueJobId(id))
}

#[tauri::command(rename_all = "camelCase")]
pub fn block_queued_jobs(
    app_state: tauri::State<'_, AppState>,
    error: String,
) -> Result<Vec<QueueEvent>, String> {
    block_pending_jobs(&app_state, error)
}

#[tauri::command(rename_all = "camelCase")]
pub fn clear_completed_queue_jobs(
    app_state: tauri::State<'_, AppState>,
) -> Result<Vec<QueueItem>, String> {
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

fn block_pending_jobs(app_state: &AppState, error: String) -> Result<Vec<QueueEvent>, String> {
    app_state.block_queued_jobs(error)
}

#[cfg(test)]
mod tests;
