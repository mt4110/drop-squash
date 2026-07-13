use std::path::{Path, PathBuf};

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
        && has_three_sample_context(&lower)
        && lower.contains("smaller")
        && lower.contains("backend")
        && lower.contains("saved")
        && lower.contains("duration")
        && lower.contains("speed ratio")
        && has_csv_path_outside_repo(result)
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

fn has_csv_path_outside_repo(value: &str) -> bool {
    value
        .split_whitespace()
        .map(csv_token)
        .filter_map(|token| {
            let path = PathBuf::from(token);
            (path.is_absolute() && path.extension().and_then(|value| value.to_str()) == Some("csv"))
                .then_some(path)
        })
        .any(|path| path.is_file() && outside_repo(&path))
}

fn csv_token(token: &str) -> &str {
    let token = token
        .trim_matches(|character: char| matches!(character, ',' | '.' | ';' | ')' | '(' | '`'));
    token
        .strip_prefix("csv=")
        .or_else(|| token.strip_prefix("CSV="))
        .or_else(|| token.strip_prefix("csv:"))
        .or_else(|| token.strip_prefix("CSV:"))
        .unwrap_or(token)
}

fn outside_repo(path: &Path) -> bool {
    let Ok(repo) = std::env::current_dir() else {
        return false;
    };
    !normalize(path).starts_with(normalize(&repo))
}

fn normalize(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                normalized.pop();
            }
            other => normalized.push(other.as_os_str()),
        }
    }
    normalized
}
