use super::{row, REQUIRED_BLOCKERS};

mod phrases;

pub(super) fn incomplete_requirements(text: &str) -> Vec<&'static str> {
    REQUIRED_BLOCKERS
        .iter()
        .copied()
        .filter(|blocker| {
            row::find(text, blocker)
                .and_then(row::completion_evidence)
                .is_some_and(|value| is_missing_requirement(blocker, value))
        })
        .collect()
}

fn is_missing_requirement(blocker: &str, value: &str) -> bool {
    !is_complete(blocker, value)
}

pub(super) fn is_complete(blocker: &str, value: &str) -> bool {
    let value = value.trim();
    !(value.is_empty()
        || super::placeholders::has_token(value)
        || required_phrases(blocker)
            .iter()
            .any(|phrase| !value.contains(phrase)))
}

fn required_phrases(blocker: &str) -> Vec<&'static str> {
    phrases::for_blocker(blocker).collect()
}

#[cfg(test)]
mod tests;
