use std::path::Path;

mod completion;
mod evidence_class;
mod evidence_ref;
mod issues;
mod placeholders;
mod records;
mod reference_urls;
mod required_blockers;
pub(super) mod row;
mod url_pairs;
mod verified_ref;

const REQUIRED_BLOCKERS: &[&str] = required_blockers::ALL;

pub(super) fn required() -> &'static [&'static str] {
    REQUIRED_BLOCKERS
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
    let mismatched_urls = url_pairs::mismatched_verified_url_pairs(&text);
    let unclassified = evidence_class::unclassified_blockers(&text);
    let issues = issues::Issues {
        missing,
        invalid,
        unproven,
        stale,
        misplaced_ref,
        incomplete,
        misplaced,
        mismatched_urls,
        unclassified,
    };
    if issues.is_empty() {
        return Ok(());
    }
    Err(issues.format(path))
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

#[cfg(test)]
mod tests;
