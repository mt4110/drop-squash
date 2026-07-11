pub(super) fn validate_result(label: &str, result: &str, missing: &mut Vec<String>) {
    let label = label.trim();
    let result = result.trim();
    if result.is_empty() {
        missing.push(format!("manual QA result is empty: {label}"));
        return;
    }
    if has_placeholder_evidence(result) || has_vague_manual_result(label, result) {
        missing.push(format!("manual QA result needs evidence: {label}"));
    }
    super::app::validate_result(label, result, missing);
    super::benchmark::validate_result(label, result, missing);
    super::license::validate_result(label, result, missing);
}

fn has_vague_manual_result(label: &str, result: &str) -> bool {
    if label.starts_with('`') {
        return false;
    }
    matches!(
        result.trim().to_ascii_lowercase().as_str(),
        "pass" | "ok" | "done" | "works" | "verified" | "observed expected behavior"
    )
}

pub(super) fn has_placeholder_evidence(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "tbd" | "todo" | "n/a" | "na" | "none" | "blocked" | "skipped"
    )
}
