pub fn redact_license_key(message: &str, license_key: &str) -> String {
    let trimmed = license_key.trim();
    if trimmed.is_empty() {
        return message.to_string();
    }
    redaction_candidates(trimmed)
        .into_iter()
        .fold(message.to_string(), |message, candidate| {
            replace_ascii_case_insensitive(&message, &candidate)
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

fn replace_ascii_case_insensitive(message: &str, candidate: &str) -> String {
    let needle = candidate.to_ascii_lowercase();
    if needle.is_empty() {
        return message.to_string();
    }
    let lower = message.to_ascii_lowercase();
    let mut output = String::with_capacity(message.len());
    let mut cursor = 0;
    while let Some(index) = lower[cursor..].find(&needle) {
        let start = cursor + index;
        let end = start + candidate.len();
        output.push_str(&message[cursor..start]);
        output.push_str("[license key]");
        cursor = end;
    }
    output.push_str(&message[cursor..]);
    output
}
