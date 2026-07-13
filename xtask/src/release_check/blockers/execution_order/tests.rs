use super::{
    duplicate_blockers, duplicate_tracks, misordered_tracks, misplaced_record_targets,
    unknown_blockers, unplanned_blockers, weak_exit_conditions,
};

#[test]
fn release_blockers_template_plans_required_rows() {
    let text = std::fs::read_to_string("../docs/release-blockers.md").unwrap();

    assert!(unplanned_blockers(&text).is_empty());
    assert!(unknown_blockers(&text).is_empty());
    assert!(duplicate_blockers(&text).is_empty());
    assert!(misordered_tracks(&text).is_empty());
    assert!(duplicate_tracks(&text).is_empty());
    assert!(misplaced_record_targets(&text).is_empty());
    assert!(weak_exit_conditions(&text).is_empty());
}

#[test]
fn reports_missing_execution_order_blocker() {
    let text = "\
| 1 | Local packaged-app proof | Packaged macOS manual QA | Public DMG evidence | `docs/manual-qa.md` |
";

    let unplanned = unplanned_blockers(text);

    assert!(unplanned.contains(&"Benchmark release set"));
}

#[test]
fn reports_missing_pricing_execution_order_blocker() {
    let text = "\
| 3 | Public web proof | Public website deployment, Refund policy finalized, Live checkout link | Production website URLs | Production website URLs |
";

    let unplanned = unplanned_blockers(text);

    assert!(unplanned.contains(&"Pricing finalized"));
}

#[test]
fn reports_blocker_in_wrong_execution_track() {
    let text = "\
| 1 | Local packaged-app proof | Packaged macOS manual QA, Benchmark release set | Public DMG evidence | `docs/manual-qa.md` |
| 2 | License sandbox proof | Public website deployment | License evidence | `docs/manual-qa.md` |
";

    let unplanned = unplanned_blockers(text);

    assert!(unplanned.contains(&"Public website deployment"));
}

#[test]
fn reports_unknown_execution_order_blocker() {
    let text = "\
| 1 | Local packaged-app proof | Packaged macOS manual QA, Extra launch task | Public DMG evidence | `docs/manual-qa.md` |
";

    let unknown = unknown_blockers(text);

    assert_eq!(unknown, vec!["Extra launch task"]);
}

#[test]
fn reports_duplicate_execution_order_blocker() {
    let text = "\
| 1 | Local packaged-app proof | Packaged macOS manual QA, Benchmark release set | Public DMG evidence | `docs/manual-qa.md` |
| 2 | License sandbox proof | Benchmark release set, Valid sandbox activation | License evidence | `docs/manual-qa.md` |
";

    let duplicates = duplicate_blockers(text);

    assert_eq!(duplicates, vec!["Benchmark release set"]);
}

#[test]
fn reports_duplicate_execution_track() {
    let text = "\
| 1 | Local packaged-app proof | Packaged macOS manual QA, Benchmark release set | Public DropSquash.dmg, manual QA, CSV outside the repo, and manual-qa-check | `docs/manual-qa.md` |
| 1 | Local packaged-app proof |  | Public DropSquash.dmg, manual QA, CSV outside the repo, and manual-qa-check | `docs/manual-qa.md` |
";

    let duplicates = duplicate_tracks(text);

    assert_eq!(duplicates, vec!["Local packaged-app proof"]);
}

#[test]
fn reports_misordered_execution_track() {
    let text = "\
| 2 | Local packaged-app proof | Packaged macOS manual QA, Benchmark release set | Public DMG evidence | `docs/manual-qa.md` |
";

    let misordered = misordered_tracks(text);

    assert!(misordered.contains(&"Local packaged-app proof"));
}

#[test]
fn reports_misplaced_execution_record_target() {
    let text = "\
| 1 | Local packaged-app proof | Packaged macOS manual QA, Benchmark release set | Public DMG evidence | Release notes |
";

    let misplaced = misplaced_record_targets(text);

    assert!(misplaced.contains(&"Local packaged-app proof"));
}

#[test]
fn reports_weak_execution_exit_condition() {
    let text = "\
| 1 | Local packaged-app proof | Packaged macOS manual QA, Benchmark release set | Run tests | `docs/manual-qa.md` |
";

    let weak = weak_exit_conditions(text);

    assert!(weak.contains(&"Local packaged-app proof"));
}

#[test]
fn reports_signing_distribution_exit_without_stapled_state() {
    let text = "\
| 4 | Signing and distribution proof | Signed DMG, Notarized and stapled DMG, Gatekeeper clean-machine open, Published checksum, Homebrew cask install | Release notes, GitHub Release, and Homebrew tap PR prove the same public DropSquash.dmg is signed, notarized, checksummed, installable, and opens without warning | Release notes and public distribution URLs |
";

    let weak = weak_exit_conditions(text);

    assert!(weak.contains(&"Signing and distribution proof"));
}

#[test]
fn reports_public_web_exit_without_download_page() {
    let text = "\
| 3 | Public web proof | Public website deployment, Refund policy finalized, Live checkout link | Production dropsquash.app release-status, privacy, pricing, support, checkout, and refund links are ready before the public beta | Production website URLs |
";

    let weak = weak_exit_conditions(text);

    assert!(weak.contains(&"Public web proof"));
}

#[test]
fn reports_placeholder_execution_exit_condition() {
    let text = "\
| 1 | Local packaged-app proof | Packaged macOS manual QA, Benchmark release set | TODO | `docs/manual-qa.md` |
";

    let weak = weak_exit_conditions(text);

    assert!(weak.contains(&"Local packaged-app proof"));
}

#[test]
fn ignores_execution_order_header_rows() {
    let text =
        "| Order | Track | Blockers | Exit condition | Record target |\n|---:|---|---|---|---|\n";

    assert!(unknown_blockers(text).is_empty());
}
