use super::{evidence_ref, row, REQUIRED_BLOCKERS};

pub(super) fn missing_release_blockers(text: &str) -> Vec<&'static str> {
    REQUIRED_BLOCKERS
        .iter()
        .copied()
        .filter(|blocker| row::find(text, blocker).is_none())
        .collect()
}

pub(super) fn invalid_status_rows(text: &str) -> Vec<&'static str> {
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

pub(super) fn unproven_verified_rows(text: &str) -> Vec<&'static str> {
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

pub(super) fn stale_blocked_rows(text: &str) -> Vec<&'static str> {
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

fn missing_evidence_reference(line: &str) -> bool {
    match row::evidence_reference(line) {
        Some(value) => !evidence_ref::is_evidence_reference(value),
        None => true,
    }
}

fn has_stale_blocked_reference(line: &str) -> bool {
    row::evidence_reference(line) != Some("TBD")
}
