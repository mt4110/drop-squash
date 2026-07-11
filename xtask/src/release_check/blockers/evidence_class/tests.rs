use super::{unclassified_blockers, ALLOWED_CLASSES};
use crate::release_check::blockers::REQUIRED_BLOCKERS;

#[test]
fn accepts_complete_evidence_classes() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .enumerate()
        .map(|(index, blocker)| {
            let class = ALLOWED_CLASSES[index % ALLOWED_CLASSES.len()];
            format!(
                "| {blocker} | {class} | Capture concrete release evidence | Record the evidence in the named location |\n"
            )
        })
        .collect::<String>();

    assert!(unclassified_blockers(&text).is_empty());
}

#[test]
fn reports_missing_evidence_classification() {
    let unclassified = unclassified_blockers("");

    assert!(unclassified.contains(&"Signed DMG"));
}

#[test]
fn reports_placeholder_next_action() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| {
            format!("| {blocker} | License sandbox | Capture concrete release evidence | TBD |\n")
        })
        .collect::<String>();

    let unclassified = unclassified_blockers(&text);

    assert!(unclassified.contains(&"Signed DMG"));
}

#[test]
fn reports_unknown_evidence_class() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| {
            format!(
                "| {blocker} | External | Capture concrete release evidence | Record the evidence in the named location |\n"
            )
        })
        .collect::<String>();

    let unclassified = unclassified_blockers(&text);

    assert!(unclassified.contains(&"Signed DMG"));
}
