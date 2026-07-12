mod groups;

pub(super) fn validate_result(label: &str, result: &str, missing: &mut Vec<String>) {
    let Some(groups) = groups::for_label(label.trim()) else {
        return;
    };
    let lower = result.to_ascii_lowercase();
    if groups
        .iter()
        .all(|group| group.iter().any(|needle| lower.contains(needle)))
        && checksum_evidence_ok(label, result)
        && privacy_receipt_evidence_ok(label, result)
        && batch_summary_evidence_ok(label, result)
        && multi_file_queue_evidence_ok(label, result)
    {
        return;
    }
    missing.push(format!(
        "manual QA {label} needs concrete packaged-app evidence"
    ));
}

fn checksum_evidence_ok(label: &str, result: &str) -> bool {
    if label != "`cargo run -p xtask -- checksum path/to/DropSquash.dmg`" {
        return true;
    }
    result
        .split(|value: char| !value.is_ascii_hexdigit())
        .any(|part| part.len() == 64)
}

fn privacy_receipt_evidence_ok(label: &str, result: &str) -> bool {
    if label != "Privacy receipt sidecar" {
        return true;
    }
    let compact = result.to_ascii_lowercase().replace(' ', "");
    compact.contains("uploaded_bytes=0")
        && compact.contains("metadata_policy=preserve")
        && compact.contains("insteadofabsolutepaths")
}

fn batch_summary_evidence_ok(label: &str, result: &str) -> bool {
    if label != "Batch summary" {
        return true;
    }
    result
        .split(|value: char| !value.is_ascii_digit())
        .filter(|part| !part.is_empty())
        .count()
        >= 5
}

fn multi_file_queue_evidence_ok(label: &str, result: &str) -> bool {
    if label != "Multi-file queue" {
        return true;
    }
    contains_number(result, "3") && contains_number(result, "1")
}

fn contains_number(result: &str, expected: &str) -> bool {
    result
        .split(|value: char| !value.is_ascii_digit())
        .any(|part| part == expected)
}
