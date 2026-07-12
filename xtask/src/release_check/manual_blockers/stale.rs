use super::{mapping, missing_result, row};

pub(super) fn manual_evidence_left_blocked(blockers: &str, manual: &str) -> Vec<&'static str> {
    mapping::MANUAL_BLOCKERS
        .iter()
        .copied()
        .filter(|(blocker, checks)| {
            row::find(blockers, blocker)
                .filter(|line| row::has_status(line, blocker, "Blocked"))
                .is_some_and(|_| checks.iter().all(|check| !missing_result(manual, check)))
        })
        .map(|(blocker, _)| blocker)
        .collect()
}

#[cfg(test)]
mod tests;
