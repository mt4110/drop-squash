use super::misplaced_verified_references;

#[test]
fn accepts_verified_reference_in_expected_record_target() {
    let text = "\
| Signed DMG | Verified | codesign output | Release notes | Release notes |
| Public website deployment | Verified | pages online | https://dropsquash.app/release-status | `https://...` |
| Live checkout link | Verified | checkout opens | https://store.lemonsqueezy.com/checkout/buy/example | `https://...` |
| Published checksum | Verified | SHA-256 attached | GitHub Release https://github.com/mt4110/drop-squash/releases/tag/v0.1.0 | GitHub Release |
| Homebrew cask install | Verified | brew install output | Homebrew tap PR https://github.com/mt4110/homebrew-tap/pull/1 | Homebrew tap PR |
| Packaged macOS manual QA | Verified | table filled | `docs/manual-qa.md` | `docs/manual-qa.md` |
";

    assert!(misplaced_verified_references(text).is_empty());
}

#[test]
fn reports_verified_reference_in_wrong_record_target() {
    let text = "\
| Signed DMG | Verified | codesign output | `docs/manual-qa.md` | Release notes |
| Homebrew cask install | Verified | brew install output | GitHub Release | Homebrew tap PR |
";

    let misplaced = misplaced_verified_references(text);

    assert!(misplaced.contains(&"Signed DMG"));
    assert!(misplaced.contains(&"Homebrew cask install"));
}

#[test]
fn reports_live_checkout_without_lemonsqueezy_checkout_url() {
    let text = "| Live checkout link | Verified | checkout opens | https://dropsquash.app/pricing | `https://...` |\n";

    let misplaced = misplaced_verified_references(text);

    assert!(misplaced.contains(&"Live checkout link"));
}

#[test]
fn reports_public_website_with_checkout_reference() {
    let text = "| Public website deployment | Verified | pages online | https://store.lemonsqueezy.com/checkout/buy/example | `https://...` |\n";

    let misplaced = misplaced_verified_references(text);

    assert!(misplaced.contains(&"Public website deployment"));
}

#[test]
fn reports_public_website_without_release_status_reference() {
    let text =
        "| Public website deployment | Verified | pages online | https://dropsquash.app/ | `https://...` |\n";

    let misplaced = misplaced_verified_references(text);

    assert!(misplaced.contains(&"Public website deployment"));
}

#[test]
fn reports_live_checkout_without_buy_reference() {
    let text =
        "| Live checkout link | Verified | checkout opens | https://store.lemonsqueezy.com/checkout | `https://...` |\n";

    let misplaced = misplaced_verified_references(text);

    assert!(misplaced.contains(&"Live checkout link"));
}

#[test]
fn reports_distribution_references_without_urls() {
    let text = "\
| Published checksum | Verified | SHA-256 attached | GitHub Release | GitHub Release |
| Homebrew cask install | Verified | brew install output | Homebrew tap PR | Homebrew tap PR |
";

    let misplaced = misplaced_verified_references(text);

    assert!(misplaced.contains(&"Published checksum"));
    assert!(misplaced.contains(&"Homebrew cask install"));
}

#[test]
fn reports_homebrew_reference_without_numeric_pull_request() {
    let text = "\
| Homebrew cask install | Verified | brew install output | Homebrew tap PR https://github.com/mt4110/homebrew-tap/pull/not-a-number | Homebrew tap PR |
";

    let misplaced = misplaced_verified_references(text);

    assert!(misplaced.contains(&"Homebrew cask install"));
}

#[test]
fn reports_distribution_references_with_wrong_repositories() {
    let text = "\
| Published checksum | Verified | SHA-256 attached | GitHub Release https://github.com/example/drop-squash/releases/tag/v0.1.0 | GitHub Release |
| Homebrew cask install | Verified | brew install output | Homebrew tap PR https://github.com/example/homebrew-tap/pull/1 | Homebrew tap PR |
";

    let misplaced = misplaced_verified_references(text);

    assert!(misplaced.contains(&"Published checksum"));
    assert!(misplaced.contains(&"Homebrew cask install"));
}

#[test]
fn ignores_blocked_rows() {
    let text = "| Signed DMG | Blocked | codesign output | TBD | Release notes |\n";

    assert!(misplaced_verified_references(text).is_empty());
}
