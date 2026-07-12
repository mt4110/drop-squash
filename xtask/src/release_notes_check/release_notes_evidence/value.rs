pub(super) fn field<'a>(label: &str, text: &'a str) -> Option<&'a str> {
    let prefix = format!("- {label}:");
    text.lines()
        .find_map(|line| line.trim().strip_prefix(&prefix).map(str::trim))
}

pub(super) fn is_placeholder(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    value.is_empty()
        || has_placeholder_token(value)
        || lower == "tbd"
        || lower == "n/a"
        || lower == "none"
        || lower == "pass"
        || lower == "ok"
        || lower == "done"
        || lower.contains("example.")
        || lower.contains("localhost")
        || lower.contains(".test/")
        || lower.ends_with(".test")
}

pub(super) fn is_concrete_evidence(value: &str) -> bool {
    !is_placeholder(value) && value.split_whitespace().count() >= 2
}

fn has_placeholder_token(value: &str) -> bool {
    value
        .split(|character: char| !character.is_ascii_alphanumeric())
        .any(|token| matches!(token.to_ascii_lowercase().as_str(), "tbd" | "todo"))
}
