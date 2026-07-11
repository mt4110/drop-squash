use super::{row, REQUIRED_BLOCKERS};

pub(super) fn incomplete_requirements(text: &str) -> Vec<&'static str> {
    REQUIRED_BLOCKERS
        .iter()
        .copied()
        .filter(|blocker| {
            row::find(text, blocker)
                .and_then(row::completion_evidence)
                .is_some_and(is_missing_requirement)
        })
        .collect()
}

fn is_missing_requirement(value: &str) -> bool {
    let value = value.trim();
    value.is_empty() || value == "TBD"
}

#[cfg(test)]
mod tests;
