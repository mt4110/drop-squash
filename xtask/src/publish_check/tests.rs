use super::{
    ensure_manual_qa_complete, ensure_website_complete, unverified_blockers,
    unverified_blockers_error,
};

#[test]
fn accepts_all_verified_blockers() {
    let text = crate::release_check::required_blockers()
        .iter()
        .map(|blocker| format!("| {blocker} | Verified | done | evidence | docs |\n"))
        .collect::<String>();

    assert!(unverified_blockers(&text).is_empty());
}

#[test]
fn reports_verified_blocker_without_evidence_reference() {
    let text = crate::release_check::required_blockers()
        .iter()
        .map(|blocker| format!("| {blocker} | Verified | done | TBD | docs |\n"))
        .collect::<String>();

    let unverified = unverified_blockers(&text);

    assert!(unverified.contains(&"Signed DMG"));
}

#[test]
fn publish_error_mentions_traceable_evidence_reference() {
    let error = unverified_blockers_error(&["Signed DMG"]);

    assert!(error.contains("traceable Evidence reference"));
    assert!(error.contains("Signed DMG"));
}

#[test]
fn reports_blocked_and_missing_blockers() {
    let text = "| Signed DMG | Blocked | evidence required | TBD | Release notes |\n";

    let unverified = unverified_blockers(text);

    assert!(unverified.contains(&"Signed DMG"));
    assert!(unverified.contains(&"Published checksum"));
}

#[test]
fn includes_license_safety_blockers() {
    let text = crate::release_check::required_blockers()
        .iter()
        .filter(|blocker| **blocker != "Invalid license key handling")
        .filter(|blocker| **blocker != "Local license forget")
        .map(|blocker| format!("| {blocker} | Verified | done | evidence | docs |\n"))
        .collect::<String>();

    let unverified = unverified_blockers(&text);

    assert!(unverified.contains(&"Invalid license key handling"));
    assert!(unverified.contains(&"Local license forget"));
}

#[test]
fn publish_requires_complete_manual_qa() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("manual-qa.md");
    std::fs::write(&path, "| App build | TBD |\n").unwrap();

    let error = ensure_manual_qa_complete(&path).unwrap_err();

    assert!(error.contains("manual QA must pass before publish"));
    assert!(error.contains("manual QA field needs evidence"));
}

#[test]
fn publish_requires_valid_website() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(directory.path().join("index.html"), "<p>DropSquash</p>").unwrap();

    let error = ensure_website_complete(directory.path()).unwrap_err();

    assert!(error.contains("website must pass before publish"));
    assert!(error.contains("pricing.html"));
}
