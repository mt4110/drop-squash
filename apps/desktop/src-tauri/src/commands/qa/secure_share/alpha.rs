use std::time::Duration;

use dropsquash_platform::{
    request_shareable_content_snapshot, SckShareableContentRequest, SckWindowCandidate,
};
use serde::Serialize;

use super::{observe_window, SecureShareObservationDto};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecureShareWindowDto {
    pub window_id: u32,
    pub title: String,
    pub width: u32,
    pub height: u32,
}

#[tauri::command(rename_all = "camelCase")]
pub async fn secure_share_alpha_windows() -> Result<Vec<SecureShareWindowDto>, String> {
    tauri::async_runtime::spawn_blocking(list_windows)
        .await
        .map_err(|error| format!("Secure Share target list failed: {error}"))?
}

#[tauri::command(rename_all = "camelCase")]
pub async fn secure_share_alpha_capture(
    window_id: u32,
) -> Result<SecureShareObservationDto, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let report = observe_window(window_id, 1_000, 10_000)?;
        if report.live_masked_frame_count == 0 || report.live_verified_pixel_count == 0 {
            return Err(
                "No text pixels were blackened. Select a window with readable text.".into(),
            );
        }
        Ok(report)
    })
    .await
    .map_err(|error| format!("Secure Share alpha capture failed: {error}"))?
}

fn list_windows() -> Result<Vec<SecureShareWindowDto>, String> {
    let snapshot = request_shareable_content_snapshot(SckShareableContentRequest {
        exclude_desktop_windows: true,
        on_screen_windows_only: true,
        timeout: Duration::from_secs(5),
    })
    .map_err(|error| error.user_message())?;
    let mut windows = snapshot
        .windows
        .iter()
        .filter(eligible)
        .map(window_dto)
        .collect::<Vec<_>>();
    windows.sort_by(|left, right| left.title.cmp(&right.title));
    Ok(windows)
}

fn eligible(window: &&SckWindowCandidate) -> bool {
    window.on_screen
        && window.active
        && window.has_owner
        && window.layer == 0
        && window.frame.width > 0
        && window.frame.height > 0
}

fn window_dto(window: &SckWindowCandidate) -> SecureShareWindowDto {
    SecureShareWindowDto {
        window_id: window.window_id,
        title: window
            .title
            .clone()
            .unwrap_or_else(|| "Untitled window".to_string()),
        width: window.frame.width,
        height: window.frame.height,
    }
}
