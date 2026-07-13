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
        .any(|part| part.len() == 64 && part.chars().all(|value| !value.is_ascii_uppercase()))
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
    result
        .split_whitespace()
        .map(|value| {
            value.trim_matches(|character: char| matches!(character, ',' | ';' | '.' | ')' | '('))
        })
        .any(is_absolute_csv_outside_repo)
}

fn is_absolute_csv_outside_repo(value: &str) -> bool {
    let path = std::path::Path::new(value);
    if !path.is_absolute() || path.extension().and_then(|value| value.to_str()) != Some("csv") {
        return false;
    }
    let Ok(repo) = std::env::current_dir() else {
        return false;
    };
    !normalize(path).starts_with(normalize(&repo))
}

fn normalize(path: &std::path::Path) -> std::path::PathBuf {
    let mut normalized = std::path::PathBuf::new();
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

fn contains_number(result: &str, expected: &str) -> bool {
    result
        .split(|value: char| !value.is_ascii_digit())
        .any(|part| part == expected)
}
