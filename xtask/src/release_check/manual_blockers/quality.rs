mod groups;
mod license;

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
        "Batch summary" => count_numbers(result) < 5 || lacks_blocked_lock_count(result),
        "Multi-file queue" => !contains_number(result, "3") || !contains_number(result, "1"),
        "Trash source policy" => lacks_verified_smaller_output(result),
        "Sandbox purchase" => !license::has_order_id(result),
        "Valid sandbox activation" | "License network failure" => {
            !has_hex_fingerprint(result)
                || !has_instance_id(result)
                || license::has_raw_key_contradiction(result)
        }
        "Empty key activation" | "Invalid key activation" | "Expired license refresh" => {
            license::has_raw_key_contradiction(result)
        }
        "`cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>`"
        | "Benchmark sample set" => crate::csv_evidence::existing_outside_repo_path(result).is_none(),
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

fn lacks_blocked_lock_count(result: &str) -> bool {
    let lower = result.to_ascii_lowercase();
    if !lower.contains("trial lock") && !lower.contains("license lock") {
        return false;
    }
    match blocked_count(&lower) {
        Some(count) => count == 0,
        None => true,
    }
}

fn blocked_count(result: &str) -> Option<u64> {
    let tokens = result
        .split(|value: char| !value.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();
    tokens.windows(2).find_map(|parts| {
        (parts[0] == "blocked")
            .then(|| parts[1].parse::<u64>().ok())
            .flatten()
    })
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
