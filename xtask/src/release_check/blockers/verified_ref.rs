use super::{records, row, REQUIRED_BLOCKERS};

pub(super) fn misplaced_verified_references(text: &str) -> Vec<&'static str> {
    REQUIRED_BLOCKERS
        .iter()
        .copied()
        .filter(|blocker| {
            row::find(text, blocker)
                .filter(|line| row::has_status(line, blocker, "Verified"))
                .and_then(row::evidence_reference)
                .is_some_and(|reference| {
                    !records::reference_matches_record_target(blocker, reference)
                })
        })
        .collect()
}

#[cfg(test)]
mod tests;
