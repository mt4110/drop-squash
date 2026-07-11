use std::path::Path;

const URL_FIELDS: [(&str, UrlKind); 5] = [
    ("Artifact URL", UrlKind::Artifact),
    ("Public website URL", UrlKind::Website),
    ("Live checkout URL", UrlKind::Checkout),
    ("GitHub Release URL", UrlKind::GitHubRelease),
    ("Homebrew tap PR URL", UrlKind::HomebrewPullRequest),
];
const EVIDENCE_FIELDS: [&str; 16] = [
    "SHA-256",
    "Git commit",
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

#[derive(Clone, Copy)]
enum UrlKind {
    Artifact,
    Website,
    Checkout,
    GitHubRelease,
    HomebrewPullRequest,
}

pub(super) fn check(path: &Path) -> Result<Vec<String>, String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    Ok(check_text(&text))
}

fn check_text(text: &str) -> Vec<String> {
    let mut errors: Vec<String> = URL_FIELDS
        .iter()
        .filter_map(|(label, kind)| validate_field(label, *kind, text))
        .collect();
    errors.extend(
        EVIDENCE_FIELDS
            .iter()
            .filter_map(|label| validate_evidence_field(label, text)),
    );
    errors
}

fn validate_field(label: &'static str, kind: UrlKind, text: &str) -> Option<String> {
    let Some(value) = field_value(label, text) else {
        return Some(format!("{label} must be present"));
    };
    if is_valid_url(kind, value) {
        return None;
    }
    Some(format!("{label} must contain a concrete production URL"))
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

fn is_valid_url(kind: UrlKind, value: &str) -> bool {
    value.starts_with("https://") && !is_placeholder(value) && matches_kind(kind, value)
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

fn matches_kind(kind: UrlKind, value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    match kind {
        UrlKind::Artifact => {
            value.starts_with("https://github.com/mt4110/drop-squash/releases/download/")
                && lower.ends_with(".dmg")
        }
        UrlKind::Website => !lower.contains("lemonsqueezy.com") && !lower.contains("checkout"),
        UrlKind::Checkout => lower.contains("lemonsqueezy.com") && lower.contains("checkout"),
        UrlKind::GitHubRelease => {
            value.starts_with("https://github.com/mt4110/drop-squash/releases/tag/")
        }
        UrlKind::HomebrewPullRequest => {
            value.starts_with("https://github.com/") && value.contains("/pull/")
        }
    }
}

#[cfg(test)]
mod tests;
