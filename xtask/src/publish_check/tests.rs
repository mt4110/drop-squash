use super::{
    ensure_manual_qa_complete, ensure_website_complete, read_release_blockers, unverified_blockers,
    unverified_blockers_error,
};

#[test]
fn accepts_all_verified_blockers() {
    let text = crate::release_check::required_blockers()
        .iter()
        .map(|blocker| {
            format!(
                "| {blocker} | Verified | done | {} | docs |\n",
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
| Signed DMG | Verified | done | Release notes | Release notes |
| Published checksum | Verified | done | GitHub Release https://github.com/mt4110/drop-squash/releases/tag/v0.1.0 | GitHub Release |
| Homebrew cask install | Verified | done | Homebrew tap PR https://github.com/mt4110/homebrew-tap/pull/1 | Homebrew tap PR |
";

    let unverified = unverified_blockers(text);

    assert!(!unverified.contains(&"Signed DMG"));
    assert!(!unverified.contains(&"Published checksum"));
    assert!(!unverified.contains(&"Homebrew cask install"));
}

#[test]
fn publish_error_mentions_traceable_evidence_reference() {
    let error = unverified_blockers_error(&["Signed DMG"]);

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
