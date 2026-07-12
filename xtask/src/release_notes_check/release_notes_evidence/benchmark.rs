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
        && has_csv_path_context(&lower)
        && has_machine_context(&lower)
        && has_os_context(&lower)
    {
        return None;
    }
    Some(
        "Benchmark sample set must mention short, medium, large, smaller outputs, CSV path outside repo, machine, and OS context"
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
    let has_threshold = lower.contains("20%") || lower.contains("20 percent");
    let has_sample = lower.contains("sample");
    let has_baseline = lower.contains("baseline");
    let has_same_machine = lower.contains("same-machine") || lower.contains("same machine");
    if has_threshold && has_sample && has_baseline && has_same_machine {
        return None;
    }
    Some(
        "Benchmark regression threshold must mention 20%, samples, same-machine comparison, and release candidate baseline"
            .to_string(),
    )
}

fn has_machine_context(value: &str) -> bool {
    value.contains("machine") || value.contains("macbook") || value.contains("mac ")
}

fn has_os_context(value: &str) -> bool {
    value.contains("macos") || value.contains("os ")
}

fn has_csv_path_context(value: &str) -> bool {
    (value.contains("outside repo") || value.contains("outside repository"))
        && value.contains(".csv")
        && value.contains('/')
}
