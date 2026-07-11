use std::path::Path;

mod completion;
mod evidence_class;
mod evidence_ref;
mod records;
pub(super) mod row;
mod verified_ref;

const REQUIRED_BLOCKERS: [&str; 12] = [
    "Packaged macOS manual QA",
    "Lemon Squeezy sandbox purchase",
    "Valid sandbox activation",
    "Invalid license key handling",
    "Local license forget",
    "Public website deployment",
    "Live checkout link",
    "Signed DMG",
    "Notarized and stapled DMG",
    "Gatekeeper clean-machine open",
    "Published checksum",
    "Homebrew cask install",
];

pub(super) fn required() -> &'static [&'static str] {
    &REQUIRED_BLOCKERS
}

pub(super) fn check_release_blockers(path: &Path) -> Result<(), String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let missing = missing_release_blockers(&text);
    let invalid = invalid_status_rows(&text);
    let unproven = unproven_verified_rows(&text);
    let stale = stale_blocked_rows(&text);
    let misplaced_ref = verified_ref::misplaced_verified_references(&text);
    let incomplete = completion::incomplete_requirements(&text);
    let misplaced = records::misplaced_record_targets(&text);
    let unclassified = evidence_class::unclassified_blockers(&text);
    if missing.is_empty()
        && invalid.is_empty()
        && unproven.is_empty()
        && stale.is_empty()
        && misplaced_ref.is_empty()
        && incomplete.is_empty()
        && misplaced.is_empty()
        && unclassified.is_empty()
    {
        return Ok(());
    }
    Err(format!(
        "{} has release blocker issues: {}{}{}{}{}{}{}{}",
        path.display(),
        join_prefix("missing ", missing),
        join_prefix(" invalid status ", invalid),
        join_prefix(" unproven verified ", unproven),
        join_prefix(" stale blocked ", stale),
        join_prefix(" misplaced verified reference ", misplaced_ref),
        join_prefix(" incomplete requirement ", incomplete),
        join_prefix(" misplaced record target ", misplaced),
        join_prefix(" unclassified ", unclassified)
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
        Some(value) => !evidence_ref::is_evidence_reference(value),
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
                .is_some_and(has_stale_blocked_reference)
        })
        .collect()
}

fn has_stale_blocked_reference(line: &str) -> bool {
    row::evidence_reference(line) != Some("TBD")
}

fn join_prefix(prefix: &str, values: Vec<&str>) -> String {
    if values.is_empty() {
        return String::new();
    }
    format!("{prefix}{}", values.join(", "))
}

#[cfg(test)]
mod tests;
