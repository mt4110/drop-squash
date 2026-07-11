pub(super) fn validate(text: &str) -> Vec<String> {
    [
        validate_sample_set(text),
        validate_regression_threshold(text),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn validate_sample_set(text: &str) -> Option<String> {
    let Some(value) = field_value("Benchmark sample set", text) else {
        return Some("Benchmark sample set must be present".to_string());
    };
    let lower = value.to_ascii_lowercase();
    if ["short", "medium", "large"]
        .iter()
        .all(|needle| lower.contains(needle))
    {
        return None;
    }
    Some("Benchmark sample set must mention short, medium, and large samples".to_string())
}

fn validate_regression_threshold(text: &str) -> Option<String> {
    let Some(value) = field_value("Benchmark regression threshold", text) else {
        return Some("Benchmark regression threshold must be present".to_string());
    };
    let lower = value.to_ascii_lowercase();
    if lower.contains("20%") || lower.contains("20 percent") {
        return None;
    }
    Some("Benchmark regression threshold must mention 20%".to_string())
}

fn field_value<'a>(label: &str, text: &'a str) -> Option<&'a str> {
    let prefix = format!("- {label}:");
    text.lines()
        .find_map(|line| line.trim().strip_prefix(&prefix).map(str::trim))
}
