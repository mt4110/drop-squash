use super::{unclassified_blockers, ALLOWED_CLASSES};
use crate::release_check::blockers::REQUIRED_BLOCKERS;

#[test]
fn accepts_complete_evidence_classes() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .enumerate()
        .map(|(index, blocker)| {
            let class = ALLOWED_CLASSES[index % ALLOWED_CLASSES.len()];
            let action = action_for(blocker);
            format!(
                "| {blocker} | {class} | {action} | Record the evidence in the named location |\n"
            )
        })
        .collect::<String>();

    assert!(unclassified_blockers(&text).is_empty());
}

#[test]
fn release_blockers_template_classifies_required_rows() {
    let text = std::fs::read_to_string("../docs/release-blockers.md").unwrap();

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
fn reports_embedded_placeholder_classification() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| {
            format!(
                "| {blocker} | Distribution | Capture concrete release evidence TBD | TODO owner |\n"
            )
        })
        .collect::<String>();

    let unclassified = unclassified_blockers(&text);

    assert!(unclassified.contains(&"Published checksum"));
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

#[test]
fn reports_packaged_manual_action_without_canonical_artifacts() {
    let text = "| Packaged macOS manual QA | Manual packaged-app | Run the packaged artifact through manual QA | `docs/manual-qa.md` |\n";

    let unclassified = unclassified_blockers(text);

    assert!(unclassified.contains(&"Packaged macOS manual QA"));
}

fn action_for(blocker: &str) -> &'static str {
    match blocker {
        "Packaged macOS manual QA" => {
            "Run packaged DropSquash.app or DropSquash.dmg through manual QA"
        }
        _ => "Capture concrete release evidence",
    }
}
