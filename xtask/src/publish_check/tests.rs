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
fn rejects_manual_qa_reference_with_inline_note() {
    let text = "\
| Packaged macOS manual QA | Verified | Tested the public DropSquash.dmg artifact with manual-qa-check evidence recorded | `docs/manual-qa.md` row 1 | `docs/manual-qa.md` |
";

    let unverified = unverified_blockers(text);

    assert!(unverified.contains(&"Packaged macOS manual QA"));
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
fn rejects_live_checkout_reference_outside_store_host() {
    let text = "| Live checkout link | Verified | done | https://dropsquash.lemonsqueezy.com/checkout/buy/abc123 | `https://...` |\n";

    let unverified = unverified_blockers(text);

    assert!(unverified.contains(&"Live checkout link"));
}

#[test]
fn rejects_live_checkout_reference_with_placeholder_buy_id() {
    let text = "| Live checkout link | Verified | done | https://store.lemonsqueezy.com/checkout/buy/example | `https://...` |\n";

    let unverified = unverified_blockers(text);

    assert!(unverified.contains(&"Live checkout link"));
}

#[test]
fn rejects_public_references_with_query_or_fragment() {
    let text = "\
| Public website deployment | Verified | done | https://dropsquash.app/release-status?source=publish | `https://...` |
| Refund policy finalized | Verified | done | https://dropsquash.app/refund#terms | `https://...` |
| Live checkout link | Verified | done | https://store.lemonsqueezy.com/checkout/buy/abc123?utm=publish | `https://...` |
| Published checksum | Verified | SHA256SUMS for public DropSquash.dmg attached | GitHub Release https://github.com/mt4110/drop-squash/releases/tag/v0.1.0#assets | GitHub Release |
| Homebrew cask install | Verified | versioned DropSquash.dmg cask includes auto_updates false and zap | Homebrew tap PR https://github.com/mt4110/homebrew-tap/pull/1?plain=1 | Homebrew tap PR |
";

    let unverified = unverified_blockers(text);

    assert!(unverified.contains(&"Public website deployment"));
    assert!(unverified.contains(&"Refund policy finalized"));
    assert!(unverified.contains(&"Live checkout link"));
    assert!(unverified.contains(&"Published checksum"));
    assert!(unverified.contains(&"Homebrew cask install"));
}

#[test]
fn rejects_public_web_references_outside_canonical_host() {
    let text = "\
| Public website deployment | Verified | Production website production URL serves release-status, privacy, pricing, support, and download | https://other.example/release-status | `https://...` |
| Refund policy finalized | Verified | Production refund policy is final and linked before checkout goes live | https://other.example/refund | `https://...` |
";

    let unverified = unverified_blockers(text);

    assert!(unverified.contains(&"Public website deployment"));
    assert!(unverified.contains(&"Refund policy finalized"));
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
| Signed DMG | Verified | `codesign` verified Developer ID for public DropSquash.dmg matching the release notes Artifact URL | Release notes | Release notes |
| Published checksum | Verified | SHA256SUMS with lowercase SHA-256 for public DropSquash.dmg matching the release notes Artifact URL attached to the GitHub Release | GitHub Release https://github.com/mt4110/drop-squash/releases/tag/v0.1.0 | GitHub Release |
| Homebrew cask install | Verified | brew install --cask installed versioned artifact DropSquash.dmg from the release notes Artifact URL with matching lowercase SHA-256, brew uninstall --cask removed it cleanly, auto_updates false, and zap | Homebrew tap PR https://github.com/mt4110/homebrew-tap/pull/1 | Homebrew tap PR |
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
fn rejects_distribution_references_with_extra_label_words() {
    let text = "\
| Published checksum | Verified | SHA256SUMS for DropSquash.dmg attached | GitHub Release approved https://github.com/mt4110/drop-squash/releases/tag/v0.1.0 | GitHub Release |
| Homebrew cask install | Verified | versioned DropSquash.dmg cask includes auto_updates false and zap | Homebrew tap PR ready https://github.com/mt4110/homebrew-tap/pull/1 | Homebrew tap PR |
";

    let unverified = unverified_blockers(text);

    assert!(unverified.contains(&"Published checksum"));
    assert!(unverified.contains(&"Homebrew cask install"));
}

#[test]
fn rejects_distribution_references_without_exact_label_spacing() {
    let text = "\
| Published checksum | Verified | SHA256SUMS for DropSquash.dmg attached | GitHub Releasehttps://github.com/mt4110/drop-squash/releases/tag/v0.1.0 | GitHub Release |
| Homebrew cask install | Verified | versioned DropSquash.dmg cask includes auto_updates false and zap | Homebrew tap PR  https://github.com/mt4110/homebrew-tap/pull/1 | Homebrew tap PR |
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
fn publish_rejects_prepared_manual_qa_draft_marker() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("manual-qa.md");
    std::fs::write(
        &path,
        "Prepared manual QA draft only. Replace this file with concrete observations.\n",
    )
    .unwrap();

    let error = ensure_manual_qa_complete(&path).unwrap_err();

    assert!(error.contains("manual QA must pass before publish"));
    assert!(error.contains("prepared draft markers"));
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

#[test]
fn publish_rejects_prepared_release_note_draft_marker() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("release-notes.md");
    std::fs::write(
        &path,
        "Prepared draft only. Replace every pending line before public release.\n",
    )
    .unwrap();

    let error = ensure_release_notes_complete(&path).unwrap_err();

    assert!(error.contains("release notes must pass before publish"));
    assert!(error.contains("prepared draft markers"));
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
        "Signed DMG" => {
            "`codesign` verified Developer ID for public DropSquash.dmg matching the release notes Artifact URL"
        }
        "Notarized and stapled DMG" => {
            "`spctl`, notary, stapled public DropSquash.dmg matching the release notes Artifact URL"
        }
        "Gatekeeper clean-machine open" => {
            "Fresh macOS account opened signed, notarized, stapled app from public DropSquash.dmg matching the release notes Artifact URL without Gatekeeper warning"
        }
        "Published checksum" => {
            "SHA256SUMS with lowercase SHA-256 for public DropSquash.dmg matching the release notes Artifact URL attached to the GitHub Release"
        }
        "Homebrew cask install" => {
            "brew install --cask installed versioned artifact DropSquash.dmg from the release notes Artifact URL with matching lowercase SHA-256, brew uninstall --cask removed it cleanly, auto_updates false, and zap"
        }
        "Packaged macOS manual QA" => {
            "Tested the public DropSquash.dmg artifact matching the release notes Artifact URL with manual-qa-check evidence recorded"
        }
        "Empty key activation" => {
            "Activate stays disabled and raw key, fingerprint, and instance are absent from local cache"
        }
        "Valid sandbox activation" => {
            "Lemon Squeezy sandbox activation reaches Pro state, Activating state disables submit, local cache was checked, 64-character lowercase hex fingerprint and `instance_id` fields are present, and raw key is absent from local cache"
        }
        "Invalid license key handling" => {
            "Activating state disables submit, friendly error, raw key, fingerprint, and instance are absent from local cache"
        }
        "License network failure" => {
            "Friendly network error, existing valid local cache was checked with 64-character lowercase hex fingerprint and `instance_id` fields remains intact, and raw key is absent from local cache"
        }
        "Expired license refresh" => {
            "expired offline grace cache shows reconnect prompt, attempted conversion is blocked before starting, and raw key is absent from local cache"
        }
        "Local license forget" => {
            "Forgetting state disables action, local cache removed, trial or locked state"
        }
        "Benchmark release set" => {
            "Release-set benchmark CSV absolute path outside repo covers backend, saved percent, duration, speed ratio, short, medium, and large samples, smaller outputs, machine/OS context, 20% regression threshold, and release candidate baseline"
        }
        "Lemon Squeezy product setup" => {
            "Sandbox product is configured for intended product DropSquash with license keys enabled and private store IDs not recorded"
        }
        "Lemon Squeezy sandbox purchase" => {
            "Sandbox checkout completed for intended product, test buyer, and order"
        }
        "Public website deployment" => {
            "Production website production URL serves release-status, privacy, pricing, support, and download"
        }
        "Refund policy finalized" => {
            "Production refund policy is final and linked before checkout goes live"
        }
        "Live checkout link" => {
            "Public pricing page opens the live `store.lemonsqueezy.com/checkout/buy/<id>` URL for tested Lemon Squeezy checkout and intended product"
        }
        _ => "concrete evidence recorded",
    }
}
