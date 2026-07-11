use super::unverified_blockers;

#[test]
fn accepts_all_verified_blockers() {
    let text = crate::release_check::required_blockers()
        .iter()
        .map(|blocker| format!("| {blocker} | Verified | done | evidence | docs |\n"))
        .collect::<String>();

    assert!(unverified_blockers(&text).is_empty());
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
