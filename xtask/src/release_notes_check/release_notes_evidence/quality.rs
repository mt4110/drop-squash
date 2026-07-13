mod groups;

pub(super) fn lacks_required_evidence(label: &str, value: &str) -> bool {
    let Some(groups) = groups::for_label(label) else {
        return super::homebrew::lacks_required_evidence(label, value).unwrap_or(false);
    };
    let lower = value.to_ascii_lowercase();
    !groups
        .iter()
        .all(|group| group.iter().any(|needle| lower.contains(needle)))
        || lacks_special_evidence(label, value)
}

fn lacks_special_evidence(label: &str, value: &str) -> bool {
    match label {
        "Queue evidence" => count_numbers(value) < 5,
        "Valid sandbox activation" | "License network failure" => {
            !has_hex_fingerprint(value)
                || !has_instance_id(value)
                || has_raw_key_contradiction(value)
        }
        "Empty key activation" | "Invalid license key handling" | "Expired license refresh" => {
            has_raw_key_contradiction(value)
        }
        "Known limitations" => has_unsupported_platform_release_claim(value),
        _ => false,
    }
}

fn count_numbers(value: &str) -> usize {
    value
        .split(|character: char| !character.is_ascii_digit())
        .filter(|part| !part.is_empty())
        .count()
}

fn has_hex_fingerprint(value: &str) -> bool {
    value
        .split(|character: char| !character.is_ascii_hexdigit())
        .any(|part| {
            part.len() == 64
                && part.chars().all(|character| {
                    character.is_ascii_hexdigit() && !character.is_ascii_uppercase()
                })
        })
}

fn has_instance_id(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.contains("instance id") || lower.contains("instance_id")
}

fn has_raw_key_contradiction(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    [
        "raw key persisted",
        "raw key present",
        "raw key stored",
        "raw key written",
        "raw key saved",
        "persisted raw key",
        "stored raw key",
        "saved raw key",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}

fn has_unsupported_platform_release_claim(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    [
        "download the windows",
        "download the linux",
        "get the windows",
        "get the linux",
        "windows version available",
        "linux version available",
        "windows app available",
        "linux app available",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}
