use std::path::Path;

const REQUIRED_BLOCKERS: [&str; 8] = [
    "Packaged macOS manual QA",
    "Lemon Squeezy sandbox purchase",
    "Valid sandbox activation",
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
        .filter(|blocker| !text.contains(&format!("| {blocker} |")))
        .collect()
}

fn invalid_status_rows(text: &str) -> Vec<&'static str> {
    REQUIRED_BLOCKERS
        .iter()
        .copied()
        .filter(|blocker| !text.lines().any(|line| has_allowed_status(line, blocker)))
        .collect()
}

fn has_allowed_status(line: &str, blocker: &str) -> bool {
    line.starts_with(&format!("| {blocker} | Blocked |"))
        || line.starts_with(&format!("| {blocker} | Verified |"))
}

fn unproven_verified_rows(text: &str) -> Vec<&'static str> {
    REQUIRED_BLOCKERS
        .iter()
        .copied()
        .filter(|blocker| {
            text.lines()
                .find(|line| line.starts_with(&format!("| {blocker} | Verified |")))
                .is_some_and(missing_evidence_reference)
        })
        .collect()
}

fn missing_evidence_reference(line: &str) -> bool {
    let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
    match cells.get(3) {
        Some(value) => !is_evidence_reference(value),
        None => true,
    }
}

fn stale_blocked_rows(text: &str) -> Vec<&'static str> {
    REQUIRED_BLOCKERS
        .iter()
        .copied()
        .filter(|blocker| {
            text.lines()
                .find(|line| line.starts_with(&format!("| {blocker} | Blocked |")))
                .is_some_and(has_evidence_reference)
        })
        .collect()
}

fn has_evidence_reference(line: &str) -> bool {
    let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
    cells
        .get(3)
        .is_some_and(|value| is_evidence_reference(value))
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
