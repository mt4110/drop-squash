pub(super) fn validate_result(label: &str, result: &str, missing: &mut Vec<String>) {
    match label.trim() {
        "Benchmark sample set" => require_sample_set(result, missing),
        "Benchmark regression threshold" => require_threshold(result, missing),
        _ => {}
    }
}

fn require_sample_set(result: &str, missing: &mut Vec<String>) {
    let lower = result.to_ascii_lowercase();
    if ["short", "medium", "large"]
        .iter()
        .all(|needle| lower.contains(needle))
        && has_machine_context(&lower)
        && has_os_context(&lower)
    {
        return;
    }
    missing.push(
        "manual QA benchmark sample set must mention short, medium, large, machine, and OS"
            .to_string(),
    );
}

fn require_threshold(result: &str, missing: &mut Vec<String>) {
    let lower = result.to_ascii_lowercase();
    if lower.contains("20%") || lower.contains("20 percent") {
        return;
    }
    missing.push("manual QA benchmark threshold must mention 20%".to_string());
}

fn has_machine_context(value: &str) -> bool {
    value.contains("machine") || value.contains("macbook") || value.contains("mac ")
}

fn has_os_context(value: &str) -> bool {
    value.contains("macos") || value.contains("os ")
}
