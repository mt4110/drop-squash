use super::{expected_target, misplaced_record_targets, reference_matches_record_target};
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

#[test]
fn rejects_public_url_references_with_query_or_fragment() {
    let cases = [
        (
            "Public website deployment",
            "https://dropsquash.app/release-status?utm=release",
        ),
        (
            "Refund policy finalized",
            "https://dropsquash.app/refund#policy",
        ),
        (
            "Live checkout link",
            "https://dropsquash.lemonsqueezy.com/checkout/buy/abc123?discount=beta",
        ),
    ];

    for (blocker, reference) in cases {
        assert!(
            !reference_matches_record_target(blocker, reference),
            "{blocker} accepted {reference}"
        );
    }
}
