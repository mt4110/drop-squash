use serde_json::json;
use tauri::AppHandle;

use super::super::alpha::recording::secure_share_alpha_record;

pub(super) fn run(app: AppHandle, window_id: u32) {
    tauri::async_runtime::spawn(async move {
        let output_dir = std::env::temp_dir().join("dropsquash-secure-share-qa");
        let value = match secure_share_alpha_record(
            app.clone(),
            window_id,
            output_dir.display().to_string(),
        )
        .await
        {
            Ok(report) => json!({ "status": "ok", "report": report }),
            Err(error) => json!({ "status": "error", "error": error }),
        };
        let _ = crate::commands::qa::record_manual_qa_event(
            "secure-share-record-window".into(),
            value.to_string(),
        );
        app.exit(0);
    });
}
