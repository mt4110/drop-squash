use super::{row_status, secrets, REQUIRED_BLOCKERS};

#[test]
fn accepts_all_required_release_blockers() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| format!("| {blocker} | Blocked | Evidence required | TBD | docs |\n"))
        .collect::<String>();

    assert!(row_status::missing_release_blockers(&text).is_empty());
    assert!(row_status::invalid_status_rows(&text).is_empty());
    assert!(row_status::unproven_verified_rows(&text).is_empty());
    assert!(row_status::stale_blocked_rows(&text).is_empty());
}

#[test]
fn release_blockers_template_contains_required_rows() {
    let text = std::fs::read_to_string("../docs/release-blockers.md").unwrap();

    assert!(row_status::missing_release_blockers(&text).is_empty());
    assert!(row_status::invalid_status_rows(&text).is_empty());
}

#[test]
fn reports_missing_release_blocker() {
    let missing = row_status::missing_release_blockers("");

    assert!(missing.contains(&"Signed DMG"));
}

#[test]
fn reports_missing_release_blocker_status() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| format!("| {blocker} | Evidence required | TBD | docs |\n"))
        .collect::<String>();
    let invalid = row_status::invalid_status_rows(&text);

    assert!(invalid.contains(&"Signed DMG"));
}

#[test]
fn reports_malformed_release_blocker_rows() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| format!("| {blocker} | Blocked | Evidence required |\n"))
        .collect::<String>();
    let invalid = row_status::invalid_status_rows(&text);

    assert!(invalid.contains(&"Signed DMG"));
}

#[test]
fn reports_verified_rows_without_evidence_reference() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| format!("| {blocker} | Verified | Evidence required | TBD | docs |\n"))
        .collect::<String>();
    let unproven = row_status::unproven_verified_rows(&text);

    assert!(unproven.contains(&"Signed DMG"));
}

#[test]
fn accepts_verified_rows_with_traceable_evidence_reference() {
    let text = "| Signed DMG | Verified | codesign output | Release notes | Release notes |\n";
    let unproven = row_status::unproven_verified_rows(text);

    assert!(unproven.is_empty());
}

#[test]
fn reports_verified_rows_with_unclosed_docs_reference() {
    let text = "| Packaged macOS manual QA | Verified | table filled | `docs/manual-qa.md | `docs/manual-qa.md` |\n";
    let unproven = row_status::unproven_verified_rows(text);

    assert!(unproven.contains(&"Packaged macOS manual QA"));
}

#[test]
fn reports_verified_rows_with_docs_reference_inline_note() {
    let text = "| Packaged macOS manual QA | Verified | table filled | `docs/manual-qa.md` row 1 | `docs/manual-qa.md` |\n";
    let unproven = row_status::unproven_verified_rows(text);

    assert!(unproven.contains(&"Packaged macOS manual QA"));
}

#[test]
fn reports_verified_rows_with_vague_evidence_reference() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| format!("| {blocker} | Verified | Evidence required | checked | docs |\n"))
        .collect::<String>();
    let unproven = row_status::unproven_verified_rows(&text);

    assert!(unproven.contains(&"Signed DMG"));
}

#[test]
fn reports_verified_rows_with_placeholder_url_reference() {
    let text =
        "| Public website deployment | Verified | Production website serves pages | https://example.com | `https://...` |\n";

    let unproven = row_status::unproven_verified_rows(text);

    assert!(unproven.contains(&"Public website deployment"));
}

#[test]
fn reports_verified_rows_with_ellipsis_url_reference() {
    let text =
        "| Public website deployment | Verified | Production website serves pages | https://.../release-status | `https://...` |\n";

    let unproven = row_status::unproven_verified_rows(text);

    assert!(unproven.contains(&"Public website deployment"));
}

#[test]
fn reports_verified_rows_with_local_only_url_reference() {
    let text = "| Public website deployment | Verified | Production website serves pages | https://192.168.0.10/release-status | `https://...` |\n";

    let unproven = row_status::unproven_verified_rows(text);

    assert!(unproven.contains(&"Public website deployment"));
}

#[test]
fn accepts_labeled_public_url_references() {
    let text = "\
| Published checksum | Verified | checksum evidence | GitHub Release HTTPS://github.com/mt4110/drop-squash/releases/tag/v0.1.0 | GitHub Release |
| Homebrew cask install | Verified | cask evidence | Homebrew tap PR https://github.com/mt4110/homebrew-tap/pull/1 | Homebrew tap PR |
";

    let unproven = row_status::unproven_verified_rows(text);

    assert!(!unproven.contains(&"Published checksum"));
    assert!(!unproven.contains(&"Homebrew cask install"));
}

#[test]
fn reports_labeled_references_without_public_url() {
    let text = "\
| Published checksum | Verified | checksum evidence | GitHub Release | GitHub Release |
| Homebrew cask install | Verified | cask evidence | Homebrew tap PR | Homebrew tap PR |
";

    let unproven = row_status::unproven_verified_rows(text);

    assert!(unproven.contains(&"Published checksum"));
    assert!(unproven.contains(&"Homebrew cask install"));
}

#[test]
fn reports_verified_rows_with_placeholder_reference_notes() {
    let text = "| Public website deployment | Verified | Production website serves pages | https://dropsquash.app/release-status TBD | `https://...` |\n";

    let unproven = row_status::unproven_verified_rows(text);

    assert!(unproven.contains(&"Public website deployment"));
}

#[test]
fn reports_verified_rows_with_tracked_url_reference() {
    let text = "\
| Public website deployment | Verified | Production website serves pages | https://dropsquash.app/release-status?utm=release | `https://...` |
| Refund policy finalized | Verified | Refund page final | https://dropsquash.app/refund#terms | `https://...` |
| Published checksum | Verified | checksum evidence | GitHub Release https://github.com/mt4110/drop-squash/releases/tag/v0.1.0?plain=1 | GitHub Release |
| Homebrew cask install | Verified | cask evidence | Homebrew tap PR https://github.com/mt4110/homebrew-tap/pull/1#discussion | Homebrew tap PR |
";

    let unproven = row_status::unproven_verified_rows(text);

    assert!(unproven.contains(&"Public website deployment"));
    assert!(unproven.contains(&"Refund policy finalized"));
    assert!(unproven.contains(&"Published checksum"));
    assert!(unproven.contains(&"Homebrew cask install"));
}

#[test]
fn reports_blocked_rows_with_evidence_reference() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| {
            format!("| {blocker} | Blocked | Evidence required | `docs/manual-qa.md` | docs |\n")
        })
        .collect::<String>();
    let stale = row_status::stale_blocked_rows(&text);

    assert!(stale.contains(&"Signed DMG"));
}

#[test]
fn reports_blocked_rows_with_vague_reference() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| format!("| {blocker} | Blocked | Evidence required | checked | docs |\n"))
        .collect::<String>();
    let stale = row_status::stale_blocked_rows(&text);

    assert!(stale.contains(&"Signed DMG"));
}

#[test]
fn reports_secret_like_release_blocker_values() {
    let text = "| Lemon Squeezy product setup | Blocked | product uses LEMON_SQUEEZY_API_KEY=private and store_id=123 | TBD | `docs/manual-qa.md` |\n";

    let values = secrets::values(text);

    assert!(values
        .iter()
        .any(|error| error.contains("secret-like value")));
}
