use std::io::Write;

use serde_json::json;
use tauri::{DragDropEvent, WindowEvent};

pub mod secure_share;

#[cfg(test)]
use dropsquash_core::SecureShareOptions;

pub fn record_manual_qa_event(kind: String, detail: String) -> Result<(), String> {
    let Some(path) = std::env::var_os("DROP_SQUASH_MANUAL_QA_EVENT_LOG") else {
        return Ok(());
    };
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|error| format!("failed to open manual QA event log: {error}"))?;
    let entry = json!({
        "kind": kind,
        "detail": detail,
        "time": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|error| format!("failed to read manual QA event time: {error}"))?
            .as_secs_f64(),
    });
    writeln!(file, "{entry}").map_err(|error| format!("failed to write manual QA event: {error}"))
}

#[cfg(test)]
pub fn secure_share_preset() -> Option<SecureShareOptions> {
    let value = std::env::var("DROP_SQUASH_QA_SECURE_SHARE").ok()?;
    serde_json::from_str(&value).ok()
}

pub fn record_window_event(event: &WindowEvent) {
    let detail = match event {
        WindowEvent::DragDrop(DragDropEvent::Enter { paths, .. }) => {
            Some(format!("enter:{}", paths.len()))
        }
        WindowEvent::DragDrop(DragDropEvent::Over { .. }) => Some("over".to_string()),
        WindowEvent::DragDrop(DragDropEvent::Drop { paths, .. }) => {
            Some(format!("drop:{}", paths.len()))
        }
        WindowEvent::DragDrop(DragDropEvent::Leave) => Some("leave".to_string()),
        _ => None,
    };
    if let Some(detail) = detail {
        let _ = record_manual_qa_event("window-drag".into(), detail);
    }
}

#[cfg(test)]
mod tests {
    use super::{record_manual_qa_event, secure_share_preset};

    #[test]
    fn skips_when_log_path_is_not_set() {
        unsafe { std::env::remove_var("DROP_SQUASH_MANUAL_QA_EVENT_LOG") };

        assert!(record_manual_qa_event("drag".into(), "enter".into()).is_ok());
    }

    #[test]
    fn appends_jsonl_when_log_path_is_set() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("events.jsonl");
        unsafe { std::env::set_var("DROP_SQUASH_MANUAL_QA_EVENT_LOG", &path) };

        record_manual_qa_event("drag".into(), "drop".into()).unwrap();

        let text = std::fs::read_to_string(path).unwrap();
        assert!(text.contains("\"kind\":\"drag\""));
        assert!(text.contains("\"detail\":\"drop\""));
        unsafe { std::env::remove_var("DROP_SQUASH_MANUAL_QA_EVENT_LOG") };
    }

    #[test]
    fn reads_secure_share_preset_from_env() {
        unsafe {
            std::env::set_var(
                "DROP_SQUASH_QA_SECURE_SHARE",
                r#"{"maskMode":"solid_black","maskRects":[{"x":1,"y":2,"width":3,"height":4}]}"#,
            )
        };
        let preset = secure_share_preset().expect("preset should parse");
        assert_eq!(preset.mask_rects.len(), 1);
        unsafe { std::env::remove_var("DROP_SQUASH_QA_SECURE_SHARE") };
    }
}
