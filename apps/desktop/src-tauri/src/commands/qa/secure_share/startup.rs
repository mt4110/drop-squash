use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use serde_json::json;
use tauri::AppHandle;

mod startup_recording;

const WINDOW_ID: &str = "DROP_SQUASH_QA_SCK_OBSERVE_WINDOW_ID";
const RECORD_WINDOW_ID: &str = "DROP_SQUASH_QA_SCK_RECORD_WINDOW_ID";
const CAPTURE_MS: &str = "DROP_SQUASH_QA_SCK_OBSERVE_CAPTURE_MS";
const TIMEOUT_MS: &str = "DROP_SQUASH_QA_SCK_OBSERVE_TIMEOUT_MS";
const QUIT_AFTER: &str = "DROP_SQUASH_QA_SCK_OBSERVE_QUIT_AFTER";

pub(super) fn run(app: AppHandle) {
    if let Some(window_id) = read_u32(RECORD_WINDOW_ID) {
        startup_recording::run(app, window_id);
        return;
    }
    let Some(window_id) = read_u32(WINDOW_ID) else {
        return;
    };
    let capture_ms = read_u64(CAPTURE_MS).unwrap_or(150);
    let timeout_ms = read_u64(TIMEOUT_MS).unwrap_or(5_000);
    let quit_after = std::env::var(QUIT_AFTER).ok().as_deref() == Some("1");
    let completed = Arc::new(AtomicBool::new(false));
    schedule_timeout(app.clone(), completed.clone(), timeout_ms, quit_after);
    let app_for_callback = app.clone();
    let completed_for_callback = completed.clone();
    if let Err(error) = app.run_on_main_thread(move || {
        super::observe_window_callback(window_id, capture_ms, timeout_ms, move |result| {
            let value = match result {
                Ok(report) => json!({ "status": "ok", "report": report }),
                Err(error) => json!({ "status": "error", "error": error }),
            };
            record_once(
                &completed_for_callback,
                &app_for_callback,
                value,
                quit_after,
            );
        });
    }) {
        record_once(
            &completed,
            &app,
            json!({ "status": "error", "error": error.to_string() }),
            quit_after,
        );
    }
}

fn schedule_timeout(app: AppHandle, completed: Arc<AtomicBool>, timeout_ms: u64, quit_after: bool) {
    tauri::async_runtime::spawn_blocking(move || {
        std::thread::sleep(std::time::Duration::from_millis(timeout_ms + 500));
        record_once(
            &completed,
            &app,
            json!({ "status": "error", "error": "Secure Share startup observation timed out" }),
            quit_after,
        );
    });
}

fn record_once(
    completed: &AtomicBool,
    app: &AppHandle,
    value: serde_json::Value,
    quit_after: bool,
) {
    if completed.swap(true, Ordering::SeqCst) {
        return;
    }
    let _ = crate::commands::qa::record_manual_qa_event(
        "secure-share-observe-window".into(),
        value.to_string(),
    );
    if quit_after {
        app.exit(0);
    }
}

fn read_u32(name: &str) -> Option<u32> {
    std::env::var(name).ok()?.parse().ok()
}

fn read_u64(name: &str) -> Option<u64> {
    std::env::var(name).ok()?.parse().ok()
}
