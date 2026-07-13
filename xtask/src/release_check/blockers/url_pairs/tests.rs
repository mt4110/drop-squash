use super::mismatched_verified_url_pairs;

#[test]
fn accepts_verified_public_site_pricing_and_refund_on_same_origin() {
    let text = "\
| Public website deployment | Verified | pages online | HTTPS://dropsquash.app/release-status | `https://...` |
| Pricing finalized | Verified | pricing final | https://dropsquash.app/pricing | `https://...` |
| Refund policy finalized | Verified | refund final | https://dropsquash.app/refund | `https://...` |
";

    assert!(mismatched_verified_url_pairs(text).is_empty());
}

#[test]
fn accepts_verified_public_site_and_refund_with_host_case_difference() {
    let text = "\
| Public website deployment | Verified | pages online | https://DropSquash.app/release-status | `https://...` |
| Refund policy finalized | Verified | refund final | https://dropsquash.app/refund | `https://...` |
";

    assert!(mismatched_verified_url_pairs(text).is_empty());
}

#[test]
fn reports_verified_refund_url_on_different_origin() {
    let text = "\
| Public website deployment | Verified | pages online | https://dropsquash.app/release-status | `https://...` |
| Refund policy finalized | Verified | refund final | https://support.dropsquash.app/refund | `https://...` |
";

    let mismatched = mismatched_verified_url_pairs(text);

    assert!(mismatched.contains(&"Refund policy finalized"));
}

#[test]
fn reports_verified_pricing_url_on_different_origin() {
    let text = "\
| Public website deployment | Verified | pages online | https://dropsquash.app/release-status | `https://...` |
| Pricing finalized | Verified | pricing final | https://buy.dropsquash.app/pricing | `https://...` |
";

    let mismatched = mismatched_verified_url_pairs(text);

    assert!(mismatched.contains(&"Pricing finalized"));
}

#[test]
fn ignores_blocked_public_site_pairs() {
    let text = "\
| Public website deployment | Blocked | pages online | TBD | `https://...` |
| Pricing finalized | Verified | pricing final | https://buy.dropsquash.app/pricing | `https://...` |
| Refund policy finalized | Verified | refund final | https://support.dropsquash.app/refund | `https://...` |
";

    assert!(mismatched_verified_url_pairs(text).is_empty());
}
