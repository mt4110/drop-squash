pub(super) fn validate_result(label: &str, result: &str, missing: &mut Vec<String>) {
    let label = label.trim();
    let result = result.trim();
    if result.is_empty() {
        missing.push(format!("manual QA result is empty: {label}"));
        return;
    }
    if has_result_placeholder(label, result) || has_vague_manual_result(label, result) {
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
        "pass"
            | "passed"
            | "ok"
            | "done"
            | "looks good"
            | "success"
            | "successful"
            | "works"
            | "verified"
            | "observed expected behavior"
    )
}

pub(super) fn has_placeholder_evidence(value: &str) -> bool {
    has_placeholder_token(value, |_| false)
}

fn has_result_placeholder(label: &str, value: &str) -> bool {
    has_placeholder_token(value, |token| {
        matches!(label, "Batch summary" | "Expired license refresh") && token == "blocked"
    })
}

fn has_placeholder_token(value: &str, allowed: impl Fn(&str) -> bool) -> bool {
    let lower = value.trim().to_ascii_lowercase();
    lower.contains("n/a")
        || lower
            .split(|character: char| !character.is_ascii_alphanumeric())
            .any(|token| {
                !allowed(token)
                    && matches!(
                        token,
                        "tbd" | "todo" | "na" | "none" | "blocked" | "skipped"
                    )
            })
}
