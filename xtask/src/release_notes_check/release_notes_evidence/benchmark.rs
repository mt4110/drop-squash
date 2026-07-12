use super::value;

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
    let Some(value) = value::field("Benchmark sample set", text) else {
        return Some("Benchmark sample set must be present".to_string());
    };
    if value::is_placeholder(value) {
        return Some("Benchmark sample set must contain concrete evidence".to_string());
    }
    let lower = value.to_ascii_lowercase();
    if ["short", "medium", "large"]
        .iter()
        .all(|needle| lower.contains(needle))
        && lower.contains("smaller")
        && has_machine_context(&lower)
        && has_os_context(&lower)
    {
        return None;
    }
    Some(
        "Benchmark sample set must mention short, medium, large, smaller outputs, machine, and OS context"
            .to_string(),
    )
}

fn validate_regression_threshold(text: &str) -> Option<String> {
    let Some(value) = value::field("Benchmark regression threshold", text) else {
        return Some("Benchmark regression threshold must be present".to_string());
    };
    if value::is_placeholder(value) {
        return Some("Benchmark regression threshold must contain concrete evidence".to_string());
    }
    let lower = value.to_ascii_lowercase();
    if lower.contains("20%") || lower.contains("20 percent") {
        return None;
    }
    Some("Benchmark regression threshold must mention 20%".to_string())
}

fn has_machine_context(value: &str) -> bool {
    value.contains("machine") || value.contains("macbook") || value.contains("mac ")
}

fn has_os_context(value: &str) -> bool {
    value.contains("macos") || value.contains("os ")
}
