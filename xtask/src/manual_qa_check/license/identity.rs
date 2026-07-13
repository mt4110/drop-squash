pub(super) fn fingerprint_evidence_ok(label: &str, result: &str) -> bool {
    if !requires_activation_identity(label) {
        return true;
    }
    result
        .split(|character: char| !character.is_ascii_hexdigit())
        .any(|part| {
            part.len() == 64
                && part.chars().all(|character| {
                    character.is_ascii_hexdigit() && !character.is_ascii_uppercase()
                })
        })
}

pub(super) fn instance_id_evidence_ok(label: &str, lower: &str) -> bool {
    !requires_activation_identity(label)
        || lower.contains("instance id")
        || lower.contains("instance_id")
}

fn requires_activation_identity(label: &str) -> bool {
    matches!(
        label,
        "Valid sandbox activation" | "License network failure"
    )
}
