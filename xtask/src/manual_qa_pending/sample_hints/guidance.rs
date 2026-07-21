use super::{SampleSet, PANEL_DIR};

pub(super) fn for_label(set: &SampleSet, label: &str) -> Option<String> {
    match label {
        "Choose recording conversion" => Some(format!(
            "sample: medium ({}) alias: {}/{} command: cargo run -p xtask -- manual-qa-open-chooser /tmp/dropsquash-qa-open-panel {} then confirm {} before Open",
            set.medium, PANEL_DIR, set.medium_alias, set.medium_alias, set.medium_alias
        )),
        "Drag-and-drop conversion" | "Privacy receipt sidecar" | "Reveal privacy receipt" | "Ask source policy" | "Trash source policy" | "Reveal output" =>
            Some(format!("sample: medium ({}) alias: {}/{} command: open /tmp/dropsquash-qa-open-panel", set.medium, PANEL_DIR, set.medium_alias)),
        "Duplicate output naming" => Some(format!("sample: duplicate ({}) alias: {}/{} command: open /tmp/dropsquash-qa-open-panel", set.medium, PANEL_DIR, set.medium_alias)),
        "Cancellation" => Some(format!(
            "sample: large ({}) alias: {}/{} command: cargo run -p xtask -- manual-qa-open-chooser /tmp/dropsquash-qa-open-panel {}",
            set.large, PANEL_DIR, set.large_alias, set.large_alias
        )),
        "Larger output" => Some(format!(
            "sample: not-smaller candidate ({}) alias: {}/qa-not-smaller.mp4 command: cargo run -p xtask -- manual-qa-open-chooser /tmp/dropsquash-qa-open-panel qa-not-smaller.mp4 and verify it still shows 'could not be made smaller', the row ends as Kept original, and original/trial/history stay unchanged before recording the result",
            set.not_smaller, PANEL_DIR
        )),
        "Multi-file queue" | "Queued job cancellation" | "Batch summary" => Some(format!(
            "sample: queue set ({}, {}, {}) aliases: {}/{}, {}/{}, {}/{} command: open /tmp/dropsquash-qa-open-panel",
            set.small, set.medium, set.large, PANEL_DIR, set.small_alias, PANEL_DIR, set.medium_alias, PANEL_DIR, set.large_alias
        )),
        "Failed conversion" => Some("sample: invalid alias /tmp/dropsquash-qa-open-panel/qa-invalid.mp4 after `cargo run -p xtask -- manual-qa-bad-input /tmp/dropsquash-qa-open-panel/qa-invalid.mp4`".to_string()),
        _ => None,
    }
}
