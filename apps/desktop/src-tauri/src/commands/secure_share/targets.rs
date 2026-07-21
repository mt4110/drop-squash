use std::time::Duration;

use dropsquash_platform::{
    request_screen_capture_access, request_shareable_content_snapshot,
    screen_capture_access_granted, SckShareableContentRequest, SckWindowCandidate,
};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecureShareWindowDto {
    pub window_id: u32,
    pub owner_pid: i32,
    pub x: i32,
    pub y: i32,
    pub title: String,
    pub width: u32,
    pub height: u32,
}

#[tauri::command]
pub async fn secure_share_recording_targets() -> Result<Vec<SecureShareWindowDto>, String> {
    tauri::async_runtime::spawn_blocking(list_windows)
        .await
        .map_err(|error| format!("Secure Share target list failed: {error}"))?
}

fn list_windows() -> Result<Vec<SecureShareWindowDto>, String> {
    if !screen_capture_access_granted() {
        request_screen_capture_access();
        return Err("Secure Share requires Screen Recording permission".to_string());
    }
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
        .map(dto)
        .collect::<Vec<_>>();
    windows.sort_by(|left, right| left.title.cmp(&right.title));
    Ok(windows)
}

fn eligible(window: &&SckWindowCandidate) -> bool {
    window.on_screen
        && window.has_owner
        && window.owner_pid.is_some()
        && window.layer == 0
        && window.frame.width > 0
        && window.frame.height > 0
}

fn dto(window: &SckWindowCandidate) -> SecureShareWindowDto {
    SecureShareWindowDto {
        window_id: window.window_id,
        owner_pid: window.owner_pid.expect("eligible window has an owner"),
        x: window.frame.x,
        y: window.frame.y,
        title: window
            .title
            .as_deref()
            .filter(|title| !title.trim().is_empty())
            .map(ToOwned::to_owned)
            .or_else(|| {
                window
                    .owner_name
                    .clone()
                    .map(|name| format!("{name} window"))
            })
            .unwrap_or_else(|| format!("Window {}", window.window_id)),
        width: window.frame.width,
        height: window.frame.height,
    }
}

#[cfg(test)]
mod tests;
