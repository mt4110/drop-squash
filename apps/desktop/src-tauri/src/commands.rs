mod config;
mod conversion;
mod dto;
mod license;
mod progress;
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
) -> std::result::Result<dto::SavedConfig, String> {
    config::save_config(app_state, output_dir, profile, output_size, source_policy)
}

#[tauri::command(rename_all = "camelCase")]
pub async fn convert(
    app_state: tauri::State<'_, crate::state::AppState>,
    window: tauri::WebviewWindow,
    input_path: String,
    output_dir: String,
    profile: Profile,
    output_size: OutputSize,
    source_policy: SourcePolicy,
) -> std::result::Result<dto::ConversionSummary, String> {
    conversion::convert(
        app_state,
        window,
        input_path,
        output_dir,
        profile,
        output_size,
        source_policy,
    )
    .await
}

#[tauri::command(rename_all = "camelCase")]
pub fn cancel_conversion(
    app_state: tauri::State<'_, crate::state::AppState>,
) -> std::result::Result<bool, String> {
    app_state.cancel_conversion()
}

#[cfg(test)]
mod tests;
