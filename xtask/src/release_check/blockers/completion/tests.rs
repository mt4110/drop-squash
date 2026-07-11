use super::incomplete_requirements;
use crate::release_check::blockers::REQUIRED_BLOCKERS;

#[test]
fn accepts_described_completion_evidence() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| {
            format!("| {blocker} | Blocked | Concrete evidence requirement | TBD | docs |\n")
        })
        .collect::<String>();

    assert!(incomplete_requirements(&text).is_empty());
}

#[test]
fn reports_tbd_completion_evidence() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| format!("| {blocker} | Blocked | TBD | TBD | docs |\n"))
        .collect::<String>();

    let incomplete = incomplete_requirements(&text);

    assert!(incomplete.contains(&"Signed DMG"));
}
