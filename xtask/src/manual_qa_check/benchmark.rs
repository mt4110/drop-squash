const BENCHMARK_COMMAND: &str =
    "`cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>`";
const SAMPLE_SET: &str = "Benchmark sample set";

pub(super) fn validate_result(label: &str, result: &str, missing: &mut Vec<String>) {
    match label.trim() {
        SAMPLE_SET => require_sample_set(result, missing),
        "Benchmark regression threshold" => require_threshold(result, missing),
        _ => {}
    }
}

pub(super) fn validate_rows(rows: &[(String, String)], missing: &mut Vec<String>) {
    let command_csv = csv_for(rows, BENCHMARK_COMMAND);
    let sample_csv = csv_for(rows, SAMPLE_SET);
    for (label, path) in [
        (BENCHMARK_COMMAND, command_csv.as_deref()),
        (SAMPLE_SET, sample_csv.as_deref()),
    ] {
        if let Some(path) = path {
            require_valid_csv(label, path, missing);
        }
    }
    if let (Some(command_csv), Some(sample_csv)) = (command_csv, sample_csv) {
        if command_csv == sample_csv {
            return;
        }
        missing.push(
            "manual QA benchmark command and sample set must reference the same CSV path"
                .to_string(),
        );
    }
}

fn require_sample_set(result: &str, missing: &mut Vec<String>) {
    let lower = result.to_ascii_lowercase();
    if ["short", "medium", "large"]
        .iter()
        .all(|needle| lower.contains(needle))
        && has_three_sample_context(&lower)
        && lower.contains("smaller")
        && lower.contains("backend")
        && lower.contains("saved")
        && lower.contains("duration")
        && lower.contains("speed ratio")
        && crate::csv_evidence::existing_outside_repo_path(result).is_some()
        && has_machine_context(&lower)
        && has_os_context(&lower)
    {
        return;
    }
    missing.push(
        "manual QA benchmark sample set must mention short, medium, large, three samples, smaller outputs, backend, saved percent, duration, speed ratio, CSV path outside repo, machine, and OS"
            .to_string(),
    );
}

fn require_threshold(result: &str, missing: &mut Vec<String>) {
    let lower = result.to_ascii_lowercase();
    let has_threshold = lower.contains("20%") || lower.contains("20 percent");
    let has_sample = lower.contains("sample");
    let has_release_candidate = lower.contains("release candidate");
    let has_baseline = lower.contains("baseline");
    let has_same_machine = lower.contains("same-machine") || lower.contains("same machine");
    if has_threshold && has_sample && has_release_candidate && has_baseline && has_same_machine {
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

fn has_three_sample_context(value: &str) -> bool {
    value
        .split(|character: char| !character.is_ascii_alphanumeric())
        .any(|part| part == "three" || part == "3")
}

fn csv_for(rows: &[(String, String)], label: &str) -> Option<std::path::PathBuf> {
    rows.iter()
        .find(|(row_label, _)| row_label == label)
        .and_then(|(_, value)| crate::csv_evidence::existing_outside_repo_path(value))
}

fn require_valid_csv(label: &str, path: &std::path::Path, missing: &mut Vec<String>) {
    if let Err(error) = crate::benchmark_csv_check::validate_path(path) {
        missing.push(format!(
            "manual QA benchmark CSV for {label} must pass benchmark-csv-check: {error}"
        ));
    }
}
