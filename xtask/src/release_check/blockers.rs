use std::path::Path;

mod row;

const REQUIRED_BLOCKERS: [&str; 10] = [
    "Packaged macOS manual QA",
    "Lemon Squeezy sandbox purchase",
    "Valid sandbox activation",
    "Public website deployment",
    "Live checkout link",
    "Signed DMG",
    "Notarized and stapled DMG",
    "Gatekeeper clean-machine open",
    "Published checksum",
    "Homebrew cask install",
];

pub(super) fn check_release_blockers(path: &Path) -> Result<(), String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let missing = missing_release_blockers(&text);
    let invalid = invalid_status_rows(&text);
    let unproven = unproven_verified_rows(&text);
    let stale = stale_blocked_rows(&text);
    if missing.is_empty() && invalid.is_empty() && unproven.is_empty() && stale.is_empty() {
        return Ok(());
    }
    Err(format!(
        "{} has release blocker issues: {}{}{}{}",
        path.display(),
        join_prefix("missing ", missing),
        join_prefix(" invalid status ", invalid),
        join_prefix(" unproven verified ", unproven),
        join_prefix(" stale blocked ", stale)
    ))
}

fn missing_release_blockers(text: &str) -> Vec<&'static str> {
    REQUIRED_BLOCKERS
        .iter()
        .copied()
        .filter(|blocker| row::find(text, blocker).is_none())
        .collect()
}

fn invalid_status_rows(text: &str) -> Vec<&'static str> {
    REQUIRED_BLOCKERS
        .iter()
        .copied()
        .filter(|blocker| {
            !row::find(text, blocker).is_some_and(|line| {
                row::has_status(line, blocker, "Blocked")
                    || row::has_status(line, blocker, "Verified")
            })
        })
        .collect()
}

fn unproven_verified_rows(text: &str) -> Vec<&'static str> {
    REQUIRED_BLOCKERS
        .iter()
        .copied()
        .filter(|blocker| {
            row::find(text, blocker)
                .filter(|line| row::has_status(line, blocker, "Verified"))
                .is_some_and(missing_evidence_reference)
        })
        .collect()
}

fn missing_evidence_reference(line: &str) -> bool {
    match row::evidence_reference(line) {
        Some(value) => !is_evidence_reference(value),
        None => true,
    }
}

fn stale_blocked_rows(text: &str) -> Vec<&'static str> {
    REQUIRED_BLOCKERS
        .iter()
        .copied()
        .filter(|blocker| {
            row::find(text, blocker)
                .filter(|line| row::has_status(line, blocker, "Blocked"))
                .is_some_and(has_evidence_reference)
        })
        .collect()
}

fn has_evidence_reference(line: &str) -> bool {
    row::evidence_reference(line).is_some_and(is_evidence_reference)
}

fn is_evidence_reference(value: &str) -> bool {
    !value.is_empty()
        && value != "TBD"
        && (value.starts_with("`docs/")
            || value.starts_with("https://")
            || matches!(
                value,
                "Release notes" | "GitHub Release" | "Homebrew tap PR"
            ))
}

fn join_prefix(prefix: &str, values: Vec<&str>) -> String {
    if values.is_empty() {
        return String::new();
    }
    format!("{prefix}{}", values.join(", "))
}

#[cfg(test)]
mod tests;
