use super::{misplaced_record_targets, EXPECTED_RECORD_TARGETS};

#[test]
fn accepts_expected_record_targets() {
    let text = EXPECTED_RECORD_TARGETS
        .iter()
        .map(|(blocker, target)| {
            format!("| {blocker} | Blocked | Evidence required | TBD | {target} |\n")
        })
        .collect::<String>();

    assert!(misplaced_record_targets(&text).is_empty());
}

#[test]
fn reports_misplaced_record_targets() {
    let text = EXPECTED_RECORD_TARGETS
        .iter()
        .map(|(blocker, _)| {
            format!("| {blocker} | Blocked | Evidence required | TBD | Release notes |\n")
        })
        .collect::<String>();

    let misplaced = misplaced_record_targets(&text);

    assert!(misplaced.contains(&"Packaged macOS manual QA"));
}
