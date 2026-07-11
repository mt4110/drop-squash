use std::path::Path;

mod consistency;
mod identity;
mod url;

const URL_FIELDS: [(&str, url::Kind); 5] = [
    ("Artifact URL", url::Kind::Artifact),
    ("Public website URL", url::Kind::Website),
    ("Live checkout URL", url::Kind::Checkout),
    ("GitHub Release URL", url::Kind::GitHubRelease),
    ("Homebrew tap PR URL", url::Kind::HomebrewPullRequest),
];
const EVIDENCE_FIELDS: [&str; 14] = [
    "`codesign`",
    "`spctl`",
    "`stapler`",
    "Apple notary log",
    "Gatekeeper clean-machine open",
    "`docs/release-blockers.md` status",
    "Manual QA record",
    "Benchmark sample set",
    "Benchmark regression threshold",
    "Lemon Squeezy sandbox purchase",
    "Lemon Squeezy sandbox activation",
    "GitHub Release checksum",
    "Homebrew tap PR",
    "Homebrew install result",
];

pub(super) fn check(path: &Path) -> Result<Vec<String>, String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    Ok(check_text(&text))
}

fn check_text(text: &str) -> Vec<String> {
    let mut errors: Vec<String> = URL_FIELDS
        .iter()
        .filter_map(|(label, kind)| validate_url_field(label, *kind, text))
        .collect();
    errors.extend(identity::validate(text));
    errors.extend(consistency::validate(text));
    errors.extend(validate_sha256(text));
    errors.extend(
        EVIDENCE_FIELDS
            .iter()
            .filter_map(|label| validate_evidence_field(label, text)),
    );
    errors
}

fn validate_url_field(label: &'static str, kind: url::Kind, text: &str) -> Option<String> {
    let Some(value) = field_value(label, text) else {
        return Some(format!("{label} must be present"));
    };
    if url::is_valid(kind, value) && !is_placeholder(value) {
        return None;
    }
    Some(format!("{label} must contain a concrete production URL"))
}

fn validate_sha256(text: &str) -> Option<String> {
    let Some(value) = field_value("SHA-256", text) else {
        return Some("SHA-256 must be present".to_string());
    };
    if value.len() == 64 && value.chars().all(|value| value.is_ascii_hexdigit()) {
        return None;
    }
    Some("SHA-256 must contain a 64-character hex checksum".to_string())
}

fn validate_evidence_field(label: &'static str, text: &str) -> Option<String> {
    let Some(value) = field_value(label, text) else {
        return Some(format!("{label} must be present"));
    };
    if is_concrete_evidence(value) {
        return None;
    }
    Some(format!("{label} must contain concrete release evidence"))
}

fn field_value<'a>(label: &str, text: &'a str) -> Option<&'a str> {
    let prefix = format!("- {label}:");
    text.lines()
        .find_map(|line| line.trim().strip_prefix(&prefix).map(str::trim))
}

fn is_placeholder(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    value.is_empty()
        || lower == "tbd"
        || lower == "n/a"
        || lower == "none"
        || lower == "pass"
        || lower == "ok"
        || lower == "done"
        || lower.contains("example.")
        || lower.contains("localhost")
        || lower.contains(".test/")
        || lower.ends_with(".test")
}

fn is_concrete_evidence(value: &str) -> bool {
    !is_placeholder(value) && value.split_whitespace().count() >= 2
}

#[cfg(test)]
mod tests;
