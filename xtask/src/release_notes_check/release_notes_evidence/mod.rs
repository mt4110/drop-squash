use std::path::Path;

mod benchmark;
mod consistency;
mod fields;
mod homebrew;
mod identity;
mod quality;
mod url;
mod value;

pub(super) fn check(path: &Path) -> Result<Vec<String>, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read release notes {}: {error}", path.display()))?;
    Ok(check_text(&text))
}

fn check_text(text: &str) -> Vec<String> {
    let mut errors = duplicate_fields(text);
    errors.extend(
        fields::URL
            .iter()
            .filter_map(|(label, kind)| validate_url_field(label, *kind, text)),
    );
    errors.extend(identity::validate(text));
    errors.extend(consistency::validate(text));
    errors.extend(benchmark::validate(text));
    errors.extend(validate_sha256(text));
    errors.extend(
        fields::EVIDENCE
            .iter()
            .filter_map(|label| validate_evidence_field(label, text)),
    );
    errors
}

fn duplicate_fields(text: &str) -> Vec<String> {
    release_note_labels()
        .into_iter()
        .filter(|label| field_count(label, text) > 1)
        .map(|label| format!("{label} must appear only once"))
        .collect()
}

fn release_note_labels() -> Vec<&'static str> {
    let mut labels = vec![
        "Version",
        "Artifact",
        "SHA-256",
        "Git commit",
        "Benchmark sample set",
        "Benchmark regression threshold",
    ];
    labels.extend(fields::URL.iter().map(|(label, _)| *label));
    labels.extend(fields::EVIDENCE);
    labels
}

fn field_count(label: &str, text: &str) -> usize {
    let prefix = format!("- {label}:");
    text.lines()
        .filter(|line| line.trim().starts_with(&prefix))
        .count()
}

fn validate_url_field(label: &'static str, kind: url::Kind, text: &str) -> Option<String> {
    let Some(value) = value::field(label, text) else {
        return Some(format!("{label} must be present"));
    };
    if url::is_valid(kind, value) && !value::is_placeholder(value) {
        return None;
    }
    Some(format!("{label} must contain a concrete production URL"))
}

fn validate_sha256(text: &str) -> Option<String> {
    let Some(value) = value::field("SHA-256", text) else {
        return Some("SHA-256 must be present".to_string());
    };
    if value.len() == 64 && value.chars().all(|value| value.is_ascii_hexdigit()) {
        return None;
    }
    Some("SHA-256 must contain a 64-character hex checksum".to_string())
}

fn validate_evidence_field(label: &'static str, text: &str) -> Option<String> {
    let Some(value) = value::field(label, text) else {
        return Some(format!("{label} must be present"));
    };
    if value::is_concrete_evidence(value) && !quality::lacks_required_evidence(label, value) {
        return None;
    }
    Some(format!("{label} must contain concrete release evidence"))
}

#[cfg(test)]
mod tests;
