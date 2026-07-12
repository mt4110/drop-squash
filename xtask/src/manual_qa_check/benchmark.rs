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
        && lower.contains("smaller")
        && has_machine_context(&lower)
        && has_os_context(&lower)
    {
        return;
    }
    missing.push(
        "manual QA benchmark sample set must mention short, medium, large, smaller outputs, machine, and OS"
            .to_string(),
    );
}

fn require_threshold(result: &str, missing: &mut Vec<String>) {
    let lower = result.to_ascii_lowercase();
    let has_threshold = lower.contains("20%") || lower.contains("20 percent");
    let has_sample = lower.contains("sample");
    let has_baseline = lower.contains("baseline");
    let has_same_machine = lower.contains("same-machine") || lower.contains("same machine");
    if has_threshold && has_sample && has_baseline && has_same_machine {
        return;
    }
    missing.push(
        "manual QA benchmark threshold must mention 20%, samples, same-machine comparison, and release candidate baseline"
            .to_string(),
    );
}

fn has_machine_context(value: &str) -> bool {
    value.contains("machine") || value.contains("macbook") || value.contains("mac ")
}

fn has_os_context(value: &str) -> bool {
    value.contains("macos") || value.contains("os ")
}
