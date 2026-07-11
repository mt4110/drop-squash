pub fn redact_license_key(message: &str, license_key: &str) -> String {
    let trimmed = license_key.trim();
    if trimmed.is_empty() {
        return message.to_string();
    }
    redaction_candidates(trimmed)
        .into_iter()
        .fold(message.to_string(), |message, candidate| {
            message.replace(&candidate, "[license key]")
        })
}

fn redaction_candidates(license_key: &str) -> Vec<String> {
    let mut candidates = vec![license_key.to_string()];
    let single_spaced = license_key.split_whitespace().collect::<Vec<_>>().join(" ");
    let compact = license_key.split_whitespace().collect::<String>();
    for candidate in [single_spaced, compact] {
        if !candidate.is_empty() && !candidates.contains(&candidate) {
            candidates.push(candidate);
        }
    }
    candidates
}
