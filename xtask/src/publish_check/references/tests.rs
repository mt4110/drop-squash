use super::mismatched;

#[test]
fn accepts_matching_release_note_references() {
    let blockers = "\
| Public website deployment | Verified | done | https://dropsquash.app/release-status | `https://...` |
| Refund policy finalized | Verified | done | https://dropsquash.app/refund | `https://...` |
| Live checkout link | Verified | done | https://store.lemonsqueezy.com/checkout/buy/abc123 | `https://...` |
| Published checksum | Verified | SHA256SUMS attached for DropSquash.dmg | GitHub Release https://github.com/mt4110/drop-squash/releases/tag/v0.1.0 | GitHub Release |
| Homebrew cask install | Verified | DropSquash.dmg cask has auto_updates false and zap | Homebrew tap PR https://github.com/mt4110/homebrew-tap/pull/1 | Homebrew tap PR |
";
    let notes = release_notes("v0.1.0", "1");

    assert!(mismatched(blockers, &notes).is_empty());
}

#[test]
fn reports_distribution_references_that_do_not_match_release_notes() {
    let blockers = "\
| Published checksum | Verified | SHA256SUMS attached for DropSquash.dmg | GitHub Release https://github.com/mt4110/drop-squash/releases/tag/v0.2.0 | GitHub Release |
| Homebrew cask install | Verified | DropSquash.dmg cask has auto_updates false and zap | Homebrew tap PR https://github.com/mt4110/homebrew-tap/pull/2 | Homebrew tap PR |
";
    let notes = release_notes("v0.1.0", "1");
    let mismatched = mismatched(blockers, &notes);

    assert!(mismatched.contains(&"Published checksum"));
    assert!(mismatched.contains(&"Homebrew cask install"));
}

#[test]
fn reports_reference_with_extra_url_before_expected_url() {
    let blockers = "\
| Published checksum | Verified | SHA256SUMS attached for DropSquash.dmg | GitHub Release https://github.com/mt4110/drop-squash/releases/tag/v0.2.0 https://github.com/mt4110/drop-squash/releases/tag/v0.1.0 | GitHub Release |
";
    let notes = release_notes("v0.1.0", "1");
    let mismatched = mismatched(blockers, &notes);

    assert!(mismatched.contains(&"Published checksum"));
}

#[test]
fn reports_reference_with_extra_url_after_expected_url() {
    let blockers = "\
| Homebrew cask install | Verified | DropSquash.dmg cask has auto_updates false and zap | Homebrew tap PR https://github.com/mt4110/homebrew-tap/pull/1 https://github.com/mt4110/homebrew-tap/pull/2 | Homebrew tap PR |
";
    let notes = release_notes("v0.1.0", "1");
    let mismatched = mismatched(blockers, &notes);

    assert!(mismatched.contains(&"Homebrew cask install"));
}

#[test]
fn ignores_blocked_references() {
    let blockers = "\
| Published checksum | Blocked | SHA256SUMS attached for DropSquash.dmg | GitHub Release https://github.com/mt4110/drop-squash/releases/tag/v0.2.0 | GitHub Release |
";
    let notes = release_notes("v0.1.0", "1");

    assert!(mismatched(blockers, &notes).is_empty());
}

fn release_notes(version: &str, pr: &str) -> String {
    format!(
        "\
- Public website URL: https://dropsquash.app/release-status
- Refund policy URL: https://dropsquash.app/refund
- Live checkout URL: https://store.lemonsqueezy.com/checkout/buy/abc123
- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/{version}
- Homebrew tap PR URL: https://github.com/mt4110/homebrew-tap/pull/{pr}
"
    )
}
