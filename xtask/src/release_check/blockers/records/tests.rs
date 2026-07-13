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
            "Pricing finalized",
            "https://dropsquash.app/pricing?utm=release",
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

#[test]
fn rejects_pricing_reference_that_points_to_checkout() {
    assert!(!reference_matches_record_target(
        "Pricing finalized",
        "https://dropsquash.app/pricing/checkout",
    ));
}

#[test]
fn rejects_pricing_reference_outside_dropsquash_domain() {
    assert!(!reference_matches_record_target(
        "Pricing finalized",
        "https://store.lemonsqueezy.com/checkout/buy/abc123",
    ));
}

#[test]
fn rejects_live_checkout_outside_store_host() {
    assert!(!reference_matches_record_target(
        "Live checkout link",
        "https://dropsquash.lemonsqueezy.com/checkout/buy/abc123",
    ));
}

#[test]
fn rejects_placeholder_live_checkout_buy_id() {
    assert!(!reference_matches_record_target(
        "Live checkout link",
        "https://store.lemonsqueezy.com/checkout/buy/example",
    ));
}
