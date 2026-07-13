use super::mismatched;

#[test]
fn accepts_matching_release_note_references() {
    let blockers = "\
| Public website deployment | Verified | done | HTTPS://dropsquash.app/release-status | `https://...` |
| Pricing finalized | Verified | done | https://dropsquash.app/pricing | `https://...` |
| Refund policy finalized | Verified | done | https://dropsquash.app/refund | `https://...` |
| Live checkout link | Verified | done | https://store.lemonsqueezy.com/checkout/buy/abc123 | `https://...` |
| Published checksum | Verified | SHA256SUMS with the lowercase SHA-256 line for public DropSquash.dmg attached | GitHub Release HTTPS://github.com/mt4110/drop-squash/releases/tag/v0.1.0 | GitHub Release |
| Homebrew cask install | Verified | brew install --cask installs versioned DropSquash.dmg with matching lowercase SHA-256, auto_updates false, and zap | Homebrew tap PR https://github.com/mt4110/homebrew-tap/pull/1 | Homebrew tap PR |
";
    let notes = release_notes("v0.1.0", "1");

    assert!(mismatched(blockers, &notes).is_empty());
}

#[test]
fn accepts_matching_release_note_references_with_host_case_difference() {
    let blockers = "\
| Public website deployment | Verified | done | https://DropSquash.app/release-status | `https://...` |
| Published checksum | Verified | SHA256SUMS with the lowercase SHA-256 line for public DropSquash.dmg attached | GitHub Release https://GitHub.com/mt4110/drop-squash/releases/tag/v0.1.0 | GitHub Release |
";
    let notes = release_notes("v0.1.0", "1");

    assert!(mismatched(blockers, &notes).is_empty());
}

#[test]
fn reports_distribution_references_that_do_not_match_release_notes() {
    let blockers = "\
| Published checksum | Verified | SHA256SUMS with the lowercase SHA-256 line for public DropSquash.dmg attached | GitHub Release https://github.com/mt4110/drop-squash/releases/tag/v0.2.0 | GitHub Release |
| Homebrew cask install | Verified | brew install --cask installs versioned DropSquash.dmg with matching lowercase SHA-256, auto_updates false, and zap | Homebrew tap PR https://github.com/mt4110/homebrew-tap/pull/2 | Homebrew tap PR |
";
    let notes = release_notes("v0.1.0", "1");
    let mismatched = mismatched(blockers, &notes);

    assert!(mismatched.contains(&"Published checksum"));
    assert!(mismatched.contains(&"Homebrew cask install"));
}

#[test]
fn reports_public_web_references_that_do_not_match_release_notes() {
    let blockers = "\
| Public website deployment | Verified | done | https://other.example/release-status | `https://...` |
| Pricing finalized | Verified | done | https://dropsquash.app/draft-pricing | `https://...` |
| Refund policy finalized | Verified | done | https://dropsquash.app/old-refund | `https://...` |
| Live checkout link | Verified | done | https://store.lemonsqueezy.com/checkout/buy/wrong | `https://...` |
";
    let notes = release_notes("v0.1.0", "1");
    let mismatched = mismatched(blockers, &notes);

    assert!(mismatched.contains(&"Public website deployment"));
    assert!(mismatched.contains(&"Pricing finalized"));
    assert!(mismatched.contains(&"Refund policy finalized"));
    assert!(mismatched.contains(&"Live checkout link"));
}

#[test]
fn reports_verified_reference_without_matching_release_note_field() {
    let blockers = "\
| Public website deployment | Verified | done | https://dropsquash.app/release-status | `https://...` |
| Published checksum | Verified | SHA256SUMS with the lowercase SHA-256 line for public DropSquash.dmg attached | GitHub Release https://github.com/mt4110/drop-squash/releases/tag/v0.1.0 | GitHub Release |
";
    let notes = "- Homebrew tap PR URL: https://github.com/mt4110/homebrew-tap/pull/1\n";

    let mismatched = mismatched(blockers, notes);

    assert!(mismatched.contains(&"Public website deployment"));
    assert!(mismatched.contains(&"Published checksum"));
}

#[test]
fn reports_verified_homebrew_reference_without_release_note_field() {
    let blockers = "\
| Homebrew cask install | Verified | brew install --cask installs versioned DropSquash.dmg with matching lowercase SHA-256, auto_updates false, and zap | Homebrew tap PR https://github.com/mt4110/homebrew-tap/pull/1 | Homebrew tap PR |
";
    let notes = "- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/v0.1.0\n";

    let mismatched = mismatched(blockers, notes);

    assert!(mismatched.contains(&"Homebrew cask install"));
}

#[test]
fn reports_reference_with_extra_url_before_expected_url() {
    let blockers = "\
| Published checksum | Verified | SHA256SUMS with the lowercase SHA-256 line for public DropSquash.dmg attached | GitHub Release https://github.com/mt4110/drop-squash/releases/tag/v0.2.0 https://github.com/mt4110/drop-squash/releases/tag/v0.1.0 | GitHub Release |
";
    let notes = release_notes("v0.1.0", "1");
    let mismatched = mismatched(blockers, &notes);

    assert!(mismatched.contains(&"Published checksum"));
}

#[test]
fn reports_reference_with_extra_url_after_expected_url() {
    let blockers = "\
| Homebrew cask install | Verified | brew install --cask installs versioned DropSquash.dmg with matching lowercase SHA-256, auto_updates false, and zap | Homebrew tap PR https://github.com/mt4110/homebrew-tap/pull/1 https://github.com/mt4110/homebrew-tap/pull/2 | Homebrew tap PR |
";
    let notes = release_notes("v0.1.0", "1");
    let mismatched = mismatched(blockers, &notes);

    assert!(mismatched.contains(&"Homebrew cask install"));
}

#[test]
fn reports_release_note_field_with_extra_url() {
    let blockers = "\
| Public website deployment | Verified | done | https://dropsquash.app/release-status | `https://...` |
";
    let notes = "\
- Public website URL: https://dropsquash.app/release-status https://dropsquash.app/refund
";

    let mismatched = mismatched(blockers, notes);

    assert!(mismatched.contains(&"Public website deployment"));
}

#[test]
fn reports_release_note_field_with_placeholder_note() {
    let blockers = "\
| Refund policy finalized | Verified | done | https://dropsquash.app/refund | `https://...` |
";
    let notes = "\
- Refund policy URL: https://dropsquash.app/refund TODO
";

    let mismatched = mismatched(blockers, notes);

    assert!(mismatched.contains(&"Refund policy finalized"));
}

#[test]
fn ignores_blocked_references() {
    let blockers = "\
| Published checksum | Blocked | SHA256SUMS with the lowercase SHA-256 line for public DropSquash.dmg attached | GitHub Release https://github.com/mt4110/drop-squash/releases/tag/v0.2.0 | GitHub Release |
";
    let notes = release_notes("v0.1.0", "1");

    assert!(mismatched(blockers, &notes).is_empty());
}

fn release_notes(version: &str, pr: &str) -> String {
    format!(
        "\
- Public website URL: https://dropsquash.app/release-status
- Pricing URL: https://dropsquash.app/pricing
- Refund policy URL: https://dropsquash.app/refund
- Live checkout URL: https://store.lemonsqueezy.com/checkout/buy/abc123
- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/{version}
- Homebrew tap PR URL: https://github.com/mt4110/homebrew-tap/pull/{pr}
"
    )
}
