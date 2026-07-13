mod groups;

use std::path::{Component, Path, PathBuf};

pub(super) fn lacks_required_evidence(check: &str, result: &str) -> bool {
    let Some(groups) = groups::for_check(check) else {
        return false;
    };
    let lower = result.to_ascii_lowercase();
    !groups
        .iter()
        .all(|group| group.iter().any(|needle| lower.contains(needle)))
        || lacks_special_evidence(check, result)
}

fn lacks_special_evidence(check: &str, result: &str) -> bool {
    match check {
        "Privacy receipt sidecar" => lacks_privacy_receipt_values(result),
        "Batch summary" => count_numbers(result) < 5,
        "Multi-file queue" => !contains_number(result, "3") || !contains_number(result, "1"),
        "Trash source policy" => lacks_verified_smaller_output(result),
        "Sandbox purchase" => !has_order_id(result),
        "Valid sandbox activation" | "License network failure" => {
            !has_hex_fingerprint(result) || !has_instance_id(result)
        }
        "`cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>`"
        | "Benchmark sample set" => !has_ready_csv_outside_repo(result),
        _ => false,
    }
}

fn lacks_privacy_receipt_values(result: &str) -> bool {
    let compact = result.to_ascii_lowercase().replace(' ', "");
    !compact.contains("uploaded_bytes=0") || !compact.contains("metadata_policy=preserve")
}

fn count_numbers(result: &str) -> usize {
    result
        .split(|value: char| !value.is_ascii_digit())
        .filter(|part| !part.is_empty())
        .count()
}

fn contains_number(result: &str, expected: &str) -> bool {
    result
        .split(|value: char| !value.is_ascii_digit())
        .any(|part| part == expected)
}

fn lacks_verified_smaller_output(result: &str) -> bool {
    let lower = result.to_ascii_lowercase();
    !lower.contains("verified") || !lower.contains("smaller")
}

fn has_order_id(result: &str) -> bool {
    result
        .to_ascii_lowercase()
        .split(|value: char| !value.is_ascii_alphanumeric())
        .collect::<Vec<_>>()
        .windows(2)
        .any(|parts| parts[0] == "order" && parts[1].chars().any(|value| value.is_ascii_digit()))
}

fn has_hex_fingerprint(result: &str) -> bool {
    result
        .split(|value: char| !value.is_ascii_hexdigit())
        .any(|part| {
            part.len() == 64
                && part
                    .chars()
                    .all(|value| value.is_ascii_hexdigit() && !value.is_ascii_uppercase())
        })
}

fn has_instance_id(result: &str) -> bool {
    let lower = result.to_ascii_lowercase();
    lower.contains("instance id") || lower.contains("instance_id")
}

fn has_ready_csv_outside_repo(result: &str) -> bool {
    result
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
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            other => normalized.push(other.as_os_str()),
        }
    }
    normalized
}
