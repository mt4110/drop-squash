pub(super) fn ok(label: &str, result: &str) -> bool {
    checksum(label, result)
        && privacy_receipt(label, result)
        && batch_summary(label, result)
        && multi_file_queue(label, result)
        && trash(label, result)
        && benchmark_csv(label, result)
}

fn checksum(label: &str, result: &str) -> bool {
    if label != "`cargo run -p xtask -- checksum path/to/DropSquash.dmg --output SHA256SUMS`" {
        return true;
    }
    result
        .split(|value: char| !value.is_ascii_hexdigit())
        .any(|part| {
            part.len() == 64
                && part.chars().all(|value| !value.is_ascii_uppercase())
                && !all_same_char(part)
        })
        && result.to_ascii_lowercase().contains("sha256sums")
}

fn privacy_receipt(label: &str, result: &str) -> bool {
    if label != "Privacy receipt sidecar" {
        return true;
    }
    let compact = result.to_ascii_lowercase().replace(' ', "");
    compact.contains("uploaded_bytes=0")
        && compact.contains("metadata_policy=preserve")
        && compact.contains("insteadofabsolutepaths")
}

fn batch_summary(label: &str, result: &str) -> bool {
    if label != "Batch summary" {
        return true;
    }
    result
        .split(|value: char| !value.is_ascii_digit())
        .filter(|part| !part.is_empty())
        .count()
        >= 5
        && has_positive_blocked_lock_count(result)
}

fn has_positive_blocked_lock_count(result: &str) -> bool {
    let lower = result.to_ascii_lowercase();
    if !lower.contains("trial lock") && !lower.contains("license lock") {
        return true;
    }
    blocked_count(&lower).is_some_and(|count| count > 0)
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

fn multi_file_queue(label: &str, result: &str) -> bool {
    if label != "Multi-file queue" {
        return true;
    }
    contains_number(result, "3") && contains_number(result, "1")
}

fn trash(label: &str, result: &str) -> bool {
    if label != "Trash source policy" {
        return true;
    }
    let lower = result.to_ascii_lowercase();
    lower.contains("verified") && lower.contains("smaller")
}

fn benchmark_csv(label: &str, result: &str) -> bool {
    if label
        != "`cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>`"
    {
        return true;
    }
    crate::csv_evidence::existing_outside_repo_path(result).is_some()
}

fn contains_number(result: &str, expected: &str) -> bool {
    result
        .split(|value: char| !value.is_ascii_digit())
        .any(|part| part == expected)
}

fn all_same_char(value: &str) -> bool {
    value
        .chars()
        .next()
        .is_some_and(|first| value.chars().all(|char| char == first))
}
