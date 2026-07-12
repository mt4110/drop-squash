use super::{unclassified_blockers, unknown_classification_rows, ALLOWED_CLASSES};
use crate::release_check::blockers::REQUIRED_BLOCKERS;

#[test]
fn accepts_complete_evidence_classes() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .enumerate()
        .map(|(index, blocker)| {
            let class = ALLOWED_CLASSES[index % ALLOWED_CLASSES.len()];
            let action = action_for(blocker);
            let owner = owner_for(blocker);
            format!("| {blocker} | {class} | {action} | {owner} |\n")
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
fn reports_unknown_evidence_classification_rows() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| {
            format!("| {blocker} | Distribution | Capture concrete release evidence | Release owner |\n")
        })
        .chain(std::iter::once(
            "| Extra launch task | Distribution | Capture concrete release evidence | Release owner |\n"
                .to_string(),
        ))
        .collect::<String>();

    let unknown = unknown_classification_rows(&text);

    assert_eq!(unknown, vec!["Extra launch task"]);
}

#[test]
fn ignores_evidence_classification_header_rows() {
    let unknown = unknown_classification_rows(
        "| Blocker | Class | Next action | Evidence owner |\n|---|---|---|---|\n",
    );

    assert!(unknown.is_empty());
}

#[test]
fn reports_packaged_manual_action_without_canonical_artifacts() {
    let text = "| Packaged macOS manual QA | Manual packaged-app | Run the packaged artifact through manual QA | `docs/manual-qa.md` |\n";

    let unclassified = unclassified_blockers(text);

    assert!(unclassified.contains(&"Packaged macOS manual QA"));
}

#[test]
fn reports_public_url_classification_without_matching_owner_field() {
    let text = "| Published checksum | Distribution | Attach SHA256SUMS containing the public DropSquash.dmg line | Release owner |\n";

    let unclassified = unclassified_blockers(text);

    assert!(unclassified.contains(&"Published checksum"));
}

fn action_for(blocker: &str) -> &'static str {
    match blocker {
        "Packaged macOS manual QA" => {
            "Run packaged DropSquash.app or DropSquash.dmg through manual QA"
        }
        _ => "Capture concrete release evidence",
    }
}

fn owner_for(blocker: &str) -> &'static str {
    match blocker {
        "Public website deployment" => "Public website URL",
        "Refund policy finalized" => "Refund policy URL",
        "Live checkout link" => "Live checkout URL",
        "Published checksum" => "GitHub Release URL",
        "Homebrew cask install" => "Homebrew tap PR URL",
        _ => "Record the evidence in the named location",
    }
}
