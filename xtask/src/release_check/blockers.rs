use std::path::Path;

mod completion;
mod duplicates;
mod evidence_class;
mod evidence_ref;
mod execution_order;
mod issues;
mod placeholders;
mod records;
mod reference_urls;
mod required_blockers;
pub(super) mod row;
mod row_status;
mod secrets;
mod url_pairs;
mod verified_ref;

const REQUIRED_BLOCKERS: &[&str] = required_blockers::ALL;

pub(super) fn required() -> &'static [&'static str] {
    REQUIRED_BLOCKERS
}

pub(super) fn completion_is_complete(blocker: &str, value: &str) -> bool {
    completion::is_complete(blocker, value)
}

pub(super) fn check_release_blockers(path: &Path) -> Result<(), String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let missing = row_status::missing_release_blockers(&text);
    let invalid = row_status::invalid_status_rows(&text);
    let unproven = row_status::unproven_verified_rows(&text);
    let stale = row_status::stale_blocked_rows(&text);
    let misplaced_ref = verified_ref::misplaced_verified_references(&text);
    let incomplete = completion::incomplete_requirements(&text);
    let misplaced = records::misplaced_record_targets(&text);
    let mismatched_urls = url_pairs::mismatched_verified_url_pairs(&text);
    let unclassified = evidence_class::unclassified_blockers(&text);
    let unknown_classifications = evidence_class::unknown_classification_rows(&text);
    let unplanned = execution_order::unplanned_blockers(&text);
    let unknown_plan_rows = execution_order::unknown_blockers(&text);
    let duplicate_plan_rows = execution_order::duplicate_blockers(&text);
    let duplicate_rows = duplicates::release_blocker_rows(&text);
    let duplicate_classifications = duplicates::classification_rows(&text);
    let secret_values = secrets::values(&text);
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
        unknown_classifications,
        unplanned,
        unknown_plan_rows,
        duplicate_plan_rows,
        duplicate_rows,
        duplicate_classifications,
        secret_values,
    };
    if issues.is_empty() {
        return Ok(());
    }
    Err(issues.format(path))
}

#[cfg(test)]
mod tests;
