use std::path::Path;

mod benchmark;
mod checksum;
mod consistency;
mod duplicates;
mod fields;
mod homebrew;
mod identity;
mod install_claims;
mod quality;
mod secrets;
mod url;
mod value;

pub(super) fn check(path: &Path) -> Result<Vec<String>, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read release notes {}: {error}", path.display()))?;
    Ok(check_text(&text))
}

fn check_text(text: &str) -> Vec<String> {
    let mut errors = duplicates::validate(text);
    errors.extend(validate_document_markers(text));
    errors.extend(
        fields::URL
            .iter()
            .filter_map(|(label, kind)| validate_url_field(label, *kind, text)),
    );
    errors.extend(identity::validate(text));
    errors.extend(install_claims::validate(text));
    errors.extend(secrets::validate(text));
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

fn validate_document_markers(text: &str) -> Option<String> {
    if text.to_ascii_lowercase().contains("prepared draft only") {
        return Some("release notes must not contain prepared draft markers".to_string());
    }
    None
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
    if value.len() == 64
        && value
            .chars()
            .all(|value| value.is_ascii_hexdigit() && !value.is_ascii_uppercase())
        && !all_same_char(value)
    {
        return None;
    }
    Some("SHA-256 must contain a real lowercase 64-character hex checksum".to_string())
}

fn all_same_char(value: &str) -> bool {
    value
        .chars()
        .next()
        .is_some_and(|first| value.chars().all(|char| char == first))
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
