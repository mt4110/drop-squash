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
        _ => false,
    }
}

fn lacks_privacy_receipt_values(result: &str) -> bool {
    let compact = result.to_ascii_lowercase().replace(' ', "");
    !compact.contains("uploaded_bytes=0") || !compact.contains("metadata_policy=preserve")
}
