mod config;
mod conversion;
mod dto;
mod license;
mod progress;
mod queue;
mod source;

use dropsquash_core::SourcePolicy;
use dropsquash_core::{OutputSize, Profile};

fn format_error(error: dropsquash_core::AppError) -> String {
    error.to_string()
}

#[tauri::command(rename_all = "camelCase")]
pub async fn load_state(
    app_state: tauri::State<'_, crate::state::AppState>,
) -> std::result::Result<dto::DropZoneState, String> {
    config::load_state(app_state).await
}

#[tauri::command(rename_all = "camelCase")]
pub fn save_config(
    app_state: tauri::State<'_, crate::state::AppState>,
    output_dir: String,
    profile: Profile,
    output_size: OutputSize,
    source_policy: SourcePolicy,
    write_privacy_receipt: bool,
) -> std::result::Result<dto::SavedConfig, String> {
    config::save_config(
        app_state,
        output_dir,
        profile,
        output_size,
        source_policy,
        write_privacy_receipt,
    )
}

#[tauri::command(rename_all = "camelCase")]
pub async fn convert(
    app_state: tauri::State<'_, crate::state::AppState>,
    window: tauri::WebviewWindow,
    request: dto::ConvertRequest,
) -> std::result::Result<dto::ConversionSummary, String> {
    conversion::convert(app_state, window, request).await
}

#[tauri::command(rename_all = "camelCase")]
pub fn cancel_conversion(
    app_state: tauri::State<'_, crate::state::AppState>,
) -> std::result::Result<bool, String> {
    app_state.cancel_conversion()
}

#[tauri::command(rename_all = "camelCase")]
pub fn enqueue_queue_job(
    app_state: tauri::State<'_, crate::state::AppState>,
    request: dto::ConvertRequest,
) -> std::result::Result<dropsquash_queue::QueueEvent, String> {
    queue::enqueue(app_state, request)
}

#[tauri::command(rename_all = "camelCase")]
pub fn start_next_queue_job(
    app_state: tauri::State<'_, crate::state::AppState>,
) -> std::result::Result<Option<dropsquash_queue::QueueEvent>, String> {
    queue::start_next(app_state)
}

#[tauri::command(rename_all = "camelCase")]
pub fn cancel_queued_job(
    app_state: tauri::State<'_, crate::state::AppState>,
    id: u64,
) -> std::result::Result<Option<dropsquash_queue::QueueEvent>, String> {
    queue::cancel_pending(app_state, id)
}

#[tauri::command(rename_all = "camelCase")]
pub fn block_queued_jobs(
    app_state: tauri::State<'_, crate::state::AppState>,
    error: String,
) -> std::result::Result<Vec<dropsquash_queue::QueueEvent>, String> {
    queue::block_pending(app_state, error)
}

#[tauri::command(rename_all = "camelCase")]
pub fn trash_original(
    source_path: String,
    output_path: String,
) -> std::result::Result<dropsquash_postprocess::SourceActionDecision, String> {
    source::trash_original(source_path, output_path)
}

#[tauri::command(rename_all = "camelCase")]
pub async fn activate_license(
    license_key: String,
) -> std::result::Result<dto::DropZoneState, String> {
    let license_state = license::activate_license(license_key)
        .await
        .map_err(format_error)?;
    config::state_for_license(license_state).await
}

#[tauri::command(rename_all = "camelCase")]
pub async fn forget_license() -> std::result::Result<dto::DropZoneState, String> {
    let license_state = license::forget_license().await.map_err(format_error)?;
    config::state_for_license(license_state).await
}

#[cfg(test)]
mod tests;
