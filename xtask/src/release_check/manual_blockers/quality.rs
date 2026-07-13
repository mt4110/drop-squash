mod groups;

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
        "Valid sandbox activation" | "License network failure" => !has_hex_fingerprint(result),
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
