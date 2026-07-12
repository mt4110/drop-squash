use super::{expected_target, misplaced_record_targets};
use crate::release_check::blockers::REQUIRED_BLOCKERS;

#[test]
fn accepts_expected_record_targets() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| {
            let target = expected_target(blocker).unwrap();
            format!("| {blocker} | Blocked | Evidence required | TBD | {target} |\n")
        })
        .collect::<String>();

    assert!(misplaced_record_targets(&text).is_empty());
}

#[test]
fn every_required_blocker_has_record_target() {
    let missing = REQUIRED_BLOCKERS
        .iter()
        .filter(|blocker| expected_target(blocker).is_none())
        .collect::<Vec<_>>();

    assert!(missing.is_empty());
}

#[test]
fn reports_misplaced_record_targets() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| {
            format!("| {blocker} | Blocked | Evidence required | TBD | Release notes |\n")
        })
        .collect::<String>();

    let misplaced = misplaced_record_targets(&text);

    assert!(misplaced.contains(&"Packaged macOS manual QA"));
}
