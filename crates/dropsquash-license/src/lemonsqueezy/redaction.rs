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
    let form_encoded = form_encode(license_key);
    for candidate in [single_spaced, compact, form_encoded] {
        if !candidate.is_empty() && !candidates.contains(&candidate) {
            candidates.push(candidate);
        }
    }
    candidates
}

fn form_encode(value: &str) -> String {
    value
        .bytes()
        .map(|byte| match byte {
            b' ' => "+".to_string(),
            b'-' | b'.' | b'_' | b'~' => (byte as char).to_string(),
            b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' => (byte as char).to_string(),
            _ => format!("%{byte:02X}"),
        })
        .collect()
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
