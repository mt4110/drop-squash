use super::{row_status, secrets, REQUIRED_BLOCKERS};

#[test]
fn accepts_all_required_release_blockers() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| format!("| {blocker} | Blocked | Evidence required | TBD | docs |\n"))
        .collect::<String>();

    assert!(row_status::missing_release_blockers(&text).is_empty());
    assert!(row_status::invalid_status_rows(&text).is_empty());
    assert!(row_status::unproven_verified_rows(&text).is_empty());
    assert!(row_status::stale_blocked_rows(&text).is_empty());
}

#[test]
fn release_blockers_template_contains_required_rows() {
    let text = std::fs::read_to_string("../docs/release-blockers.md").unwrap();

    assert!(row_status::missing_release_blockers(&text).is_empty());
    assert!(row_status::invalid_status_rows(&text).is_empty());
}

#[test]
fn reports_missing_release_blocker() {
    let missing = row_status::missing_release_blockers("");

    assert!(missing.contains(&"Signed DMG"));
}

#[test]
fn reports_missing_release_blocker_status() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| format!("| {blocker} | Evidence required | TBD | docs |\n"))
        .collect::<String>();
    let invalid = row_status::invalid_status_rows(&text);

    assert!(invalid.contains(&"Signed DMG"));
}

#[test]
fn reports_malformed_release_blocker_rows() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| format!("| {blocker} | Blocked | Evidence required |\n"))
        .collect::<String>();
    let invalid = row_status::invalid_status_rows(&text);

    assert!(invalid.contains(&"Signed DMG"));
}

#[test]
fn reports_verified_rows_without_evidence_reference() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| format!("| {blocker} | Verified | Evidence required | TBD | docs |\n"))
        .collect::<String>();
    let unproven = row_status::unproven_verified_rows(&text);

    assert!(unproven.contains(&"Signed DMG"));
}

#[test]
fn accepts_verified_rows_with_traceable_evidence_reference() {
    let text = "| Signed DMG | Verified | codesign output | Release notes | Release notes |\n";
    let unproven = row_status::unproven_verified_rows(text);

    assert!(unproven.is_empty());
}

#[test]
fn reports_verified_rows_with_vague_evidence_reference() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| format!("| {blocker} | Verified | Evidence required | checked | docs |\n"))
        .collect::<String>();
    let unproven = row_status::unproven_verified_rows(&text);

    assert!(unproven.contains(&"Signed DMG"));
}

#[test]
fn reports_verified_rows_with_placeholder_url_reference() {
    let text =
        "| Public website deployment | Verified | Production website serves pages | https://example.com | `https://...` |\n";

    let unproven = row_status::unproven_verified_rows(text);

    assert!(unproven.contains(&"Public website deployment"));
}

#[test]
fn reports_verified_rows_with_placeholder_reference_notes() {
    let text = "| Public website deployment | Verified | Production website serves pages | https://dropsquash.app/release-status TBD | `https://...` |\n";

    let unproven = row_status::unproven_verified_rows(text);

    assert!(unproven.contains(&"Public website deployment"));
}

#[test]
fn reports_blocked_rows_with_evidence_reference() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| {
            format!("| {blocker} | Blocked | Evidence required | `docs/manual-qa.md` | docs |\n")
        })
        .collect::<String>();
    let stale = row_status::stale_blocked_rows(&text);

    assert!(stale.contains(&"Signed DMG"));
}

#[test]
fn reports_blocked_rows_with_vague_reference() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| format!("| {blocker} | Blocked | Evidence required | checked | docs |\n"))
        .collect::<String>();
    let stale = row_status::stale_blocked_rows(&text);

    assert!(stale.contains(&"Signed DMG"));
}

#[test]
fn reports_secret_like_release_blocker_values() {
    let text = "| Lemon Squeezy product setup | Blocked | product uses LEMON_SQUEEZY_API_KEY=private | TBD | `docs/manual-qa.md` |\n";

    let values = secrets::values(text);

    assert!(values
        .iter()
        .any(|error| error.contains("secret-like value")));
}
