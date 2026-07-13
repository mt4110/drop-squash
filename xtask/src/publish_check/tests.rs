use super::{
    ensure_manual_qa_complete, ensure_release_notes_complete, ensure_website_complete,
    read_release_blockers, unverified_blockers, unverified_blockers_error,
};

#[test]
fn accepts_all_verified_blockers() {
    let text = crate::release_check::required_blockers()
        .iter()
        .map(|blocker| {
            format!(
                "| {blocker} | Verified | {} | {} | docs |\n",
                evidence(blocker),
                reference(blocker)
            )
        })
        .collect::<String>();

    assert!(unverified_blockers(&text).is_empty());
}

#[test]
fn rejects_verified_blockers_with_wrong_reference_kind() {
    let text = "\
| Signed DMG | Verified | done | `docs/manual-qa.md` | Release notes |
| Public website deployment | Verified | done | `docs/manual-qa.md` | `https://...` |
| Published checksum | Verified | done | `docs/manual-qa.md` | GitHub Release |
| Homebrew cask install | Verified | done | `docs/manual-qa.md` | Homebrew tap PR |
";

    let unverified = unverified_blockers(text);

    assert!(unverified.contains(&"Signed DMG"));
    assert!(unverified.contains(&"Public website deployment"));
    assert!(unverified.contains(&"Published checksum"));
    assert!(unverified.contains(&"Homebrew cask install"));
}

#[test]
fn rejects_live_checkout_reference_with_extra_path_after_buy_id() {
    let text = "| Live checkout link | Verified | done | https://store.lemonsqueezy.com/checkout/buy/abc123/extra | `https://...` |\n";

    let unverified = unverified_blockers(text);

    assert!(unverified.contains(&"Live checkout link"));
}

#[test]
fn rejects_live_checkout_reference_with_prefixed_checkout_path() {
    let text = "| Live checkout link | Verified | done | https://store.lemonsqueezy.com/store/checkout/buy/abc123 | `https://...` |\n";

    let unverified = unverified_blockers(text);

    assert!(unverified.contains(&"Live checkout link"));
}

#[test]
fn reports_verified_blocker_without_evidence_reference() {
    let text = crate::release_check::required_blockers()
        .iter()
        .map(|blocker| format!("| {blocker} | Verified | done | evidence | docs |\n"))
        .collect::<String>();

    let unverified = unverified_blockers(&text);

    assert!(unverified.contains(&"Signed DMG"));
}

#[test]
fn accepts_public_release_evidence_references() {
    let text = "\
| Signed DMG | Verified | `codesign` verified Developer ID for public DropSquash.dmg | Release notes | Release notes |
| Published checksum | Verified | SHA256SUMS with lowercase SHA-256 for public DropSquash.dmg attached to the GitHub Release | GitHub Release https://github.com/mt4110/drop-squash/releases/tag/v0.1.0 | GitHub Release |
| Homebrew cask install | Verified | brew install --cask installed versioned artifact DropSquash.dmg with matching lowercase SHA-256, auto_updates false, and zap | Homebrew tap PR https://github.com/mt4110/homebrew-tap/pull/1 | Homebrew tap PR |
";

    let unverified = unverified_blockers(text);

    assert!(!unverified.contains(&"Signed DMG"));
    assert!(!unverified.contains(&"Published checksum"));
    assert!(!unverified.contains(&"Homebrew cask install"));
}

#[test]
fn rejects_verified_blocker_with_weak_completion_evidence() {
    let text = "\
| Signed DMG | Verified | done | Release notes | Release notes |
";

    let unverified = unverified_blockers(text);

    assert!(unverified.contains(&"Signed DMG"));
}

#[test]
fn rejects_github_release_reference_without_semver_tag() {
    let text = "\
| Published checksum | Verified | SHA256SUMS for DropSquash.dmg attached | GitHub Release https://github.com/mt4110/drop-squash/releases/tag/v0.1 | GitHub Release |
";

    let unverified = unverified_blockers(text);

    assert!(unverified.contains(&"Published checksum"));
}

#[test]
fn rejects_distribution_references_with_placeholder_notes() {
    let text = "\
| Published checksum | Verified | SHA256SUMS for DropSquash.dmg attached | GitHub Release https://github.com/mt4110/drop-squash/releases/tag/v0.1.0 TBD | GitHub Release |
| Homebrew cask install | Verified | versioned DropSquash.dmg cask includes auto_updates false and zap | Homebrew tap PR https://github.com/mt4110/homebrew-tap/pull/1 TODO | Homebrew tap PR |
";

    let unverified = unverified_blockers(text);

    assert!(unverified.contains(&"Published checksum"));
    assert!(unverified.contains(&"Homebrew cask install"));
}

#[test]
fn rejects_distribution_references_with_multiple_urls() {
    let text = "\
| Published checksum | Verified | SHA256SUMS for DropSquash.dmg attached | GitHub Release https://github.com/mt4110/drop-squash/releases/tag/v0.1.0 https://github.com/mt4110/drop-squash/releases/tag/v0.2.0 | GitHub Release |
| Homebrew cask install | Verified | versioned DropSquash.dmg cask includes auto_updates false and zap | Homebrew tap PR https://github.com/mt4110/homebrew-tap/pull/1 https://github.com/mt4110/homebrew-tap/pull/2 | Homebrew tap PR |
";

    let unverified = unverified_blockers(text);

    assert!(unverified.contains(&"Published checksum"));
    assert!(unverified.contains(&"Homebrew cask install"));
}

#[test]
fn rejects_verified_references_with_imposter_hosts() {
    let text = "\
| Live checkout link | Verified | checkout opens | https://lemonsqueezy.com.evil/checkout/buy/abc123 | `https://...` |
| Published checksum | Verified | SHA256SUMS for DropSquash.dmg attached | GitHub Release https://github.com.evil/mt4110/drop-squash/releases/tag/v0.1.0 | GitHub Release |
| Homebrew cask install | Verified | versioned DropSquash.dmg cask includes auto_updates false and zap | Homebrew tap PR https://github.com.evil/mt4110/homebrew-tap/pull/1 | Homebrew tap PR |
";

    let unverified = unverified_blockers(text);

    assert!(unverified.contains(&"Live checkout link"));
    assert!(unverified.contains(&"Published checksum"));
    assert!(unverified.contains(&"Homebrew cask install"));
}

#[test]
fn publish_error_mentions_completion_evidence_and_reference() {
    let error = unverified_blockers_error(&["Signed DMG"]);

    assert!(error.contains("concrete Completion evidence"));
    assert!(error.contains("traceable Evidence reference"));
    assert!(error.contains("Signed DMG"));
}

#[test]
fn reports_missing_release_blockers_path() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("missing-release-blockers.md");

    let error = read_release_blockers(&path).unwrap_err();

    assert!(error.contains("failed to read release blockers"));
    assert!(error.contains("missing-release-blockers.md"));
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

#[test]
fn publish_requires_complete_manual_qa() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("manual-qa.md");
    std::fs::write(&path, "| App build | TBD |\n").unwrap();

    let error = ensure_manual_qa_complete(&path).unwrap_err();

    assert!(error.contains("manual QA must pass before publish"));
    assert!(error.contains("manual QA field needs evidence"));
}

#[test]
fn publish_requires_valid_website() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(directory.path().join("index.html"), "<p>DropSquash</p>").unwrap();

    let error = ensure_website_complete(directory.path()).unwrap_err();

    assert!(error.contains("website must pass before publish"));
    assert!(error.contains("pricing.html"));
}

#[test]
fn publish_requires_final_release_notes() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("release-notes.md");
    std::fs::write(
        &path,
        "- GitHub Release checksum: pending upload; replace this line with public release evidence\n",
    )
    .unwrap();

    let error = ensure_release_notes_complete(&path).unwrap_err();

    assert!(error.contains("release notes must pass before publish"));
    assert!(error.contains("GitHub Release checksum"));
}

fn reference(blocker: &str) -> &'static str {
    match blocker {
        "Signed DMG" | "Notarized and stapled DMG" => "Release notes",
        "Published checksum" => {
            "GitHub Release https://github.com/mt4110/drop-squash/releases/tag/v0.1.0"
        }
        "Homebrew cask install" => "Homebrew tap PR https://github.com/mt4110/homebrew-tap/pull/1",
        "Public website deployment" => "https://dropsquash.app/release-status",
        "Refund policy finalized" => "https://dropsquash.app/refund",
        "Live checkout link" => "https://store.lemonsqueezy.com/checkout/buy/abc123",
        _ => "`docs/manual-qa.md`",
    }
}

fn evidence(blocker: &str) -> &'static str {
    match blocker {
        "Signed DMG" => "`codesign` verified Developer ID for public DropSquash.dmg",
        "Notarized and stapled DMG" => "`spctl`, notary, stapled public DropSquash.dmg",
        "Gatekeeper clean-machine open" => {
            "Fresh macOS account opened signed, notarized, stapled app from public DropSquash.dmg without Gatekeeper warning"
        }
        "Published checksum" => {
            "SHA256SUMS with lowercase SHA-256 for public DropSquash.dmg attached to the GitHub Release"
        }
        "Homebrew cask install" => {
            "brew install --cask installed versioned artifact DropSquash.dmg with matching lowercase SHA-256, auto_updates false, and zap"
        }
        "Packaged macOS manual QA" => {
            "Tested the public DropSquash.dmg artifact with manual-qa-check evidence recorded"
        }
        "Empty key activation" => {
            "Activate stays disabled and raw key, fingerprint, and instance are absent from local cache"
        }
        "Valid sandbox activation" => {
            "Lemon Squeezy sandbox activation reaches Pro state, Activating state disables submit, 64-character lowercase hex fingerprint and `instance_id` fields are present, and raw key is absent from local cache"
        }
        "Invalid license key handling" => {
            "Activating state disables submit, friendly error, raw key, fingerprint, and instance are absent from local cache"
        }
        "License network failure" => {
            "Friendly network error, existing valid local cache with 64-character lowercase hex fingerprint and `instance_id` fields remains intact, and raw key is absent from local cache"
        }
        "Expired license refresh" => {
            "Expired offline grace cache shows reconnect prompt, conversion is blocked before starting, and raw key is absent from local cache"
        }
        "Local license forget" => {
            "Forgetting state disables action, local cache removed, trial or locked state"
        }
        "Benchmark release set" => {
            "Release-set benchmark CSV absolute path outside repo covers backend, saved percent, duration, speed ratio, short, medium, and large samples, smaller outputs, machine/OS context, and 20% regression threshold"
        }
        "Lemon Squeezy product setup" => {
            "Sandbox product is configured for intended product DropSquash with license keys enabled"
        }
        "Lemon Squeezy sandbox purchase" => {
            "Sandbox checkout completed for intended product, test buyer, and order"
        }
        "Public website deployment" => {
            "Production website serves release-status, privacy, pricing, support, and download"
        }
        "Refund policy finalized" => {
            "Production refund policy final before checkout goes live"
        }
        "Live checkout link" => {
            "Public pricing page opens tested Lemon Squeezy checkout for intended product"
        }
        _ => "concrete evidence recorded",
    }
}
