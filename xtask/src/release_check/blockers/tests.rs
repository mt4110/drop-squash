use super::{
    invalid_status_rows, missing_release_blockers, unproven_verified_rows, REQUIRED_BLOCKERS,
};

#[test]
fn accepts_all_required_release_blockers() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| format!("| {blocker} | Blocked | Evidence required |\n"))
        .collect::<String>();

    assert!(missing_release_blockers(&text).is_empty());
    assert!(invalid_status_rows(&text).is_empty());
    assert!(unproven_verified_rows(&text).is_empty());
}

#[test]
fn reports_missing_release_blocker() {
    let missing = missing_release_blockers("");

    assert!(missing.contains(&"Signed DMG"));
}

#[test]
fn reports_missing_release_blocker_status() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| format!("| {blocker} | Evidence required |\n"))
        .collect::<String>();
    let invalid = invalid_status_rows(&text);

    assert!(invalid.contains(&"Signed DMG"));
}

#[test]
fn reports_verified_rows_without_evidence_reference() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| format!("| {blocker} | Verified | Evidence required | TBD |\n"))
        .collect::<String>();
    let unproven = unproven_verified_rows(&text);

    assert!(unproven.contains(&"Signed DMG"));
}

#[test]
fn accepts_verified_rows_with_traceable_evidence_reference() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| {
            format!("| {blocker} | Verified | Evidence required | `docs/manual-qa.md` |\n")
        })
        .collect::<String>();
    let unproven = unproven_verified_rows(&text);

    assert!(unproven.is_empty());
}

#[test]
fn reports_verified_rows_with_vague_evidence_reference() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| format!("| {blocker} | Verified | Evidence required | checked |\n"))
        .collect::<String>();
    let unproven = unproven_verified_rows(&text);

    assert!(unproven.contains(&"Signed DMG"));
}
