use super::{unverified_blockers, BLOCKERS};

#[test]
fn accepts_all_verified_blockers() {
    let text = BLOCKERS
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
