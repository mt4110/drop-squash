use super::{is_paid_beta_alias, normalize_track, parse, parse_args, render, scope::Scope};

#[test]
fn reports_counts_tracks_and_next_track() {
    let report = parse::report(
        "\
| Blocker | Status | Completion evidence | Evidence reference | Record in |
|---|---|---|---|---|
| Packaged macOS manual QA | Verified | done | docs | docs |
| Benchmark release set | Blocked | evidence | TBD | docs |
| Public website deployment | Blocked | evidence | TBD | url |

## Evidence Classes

| Blocker | Class | Next action | Evidence owner |
|---|---|---|---|
| Benchmark release set | Benchmark | Run release-set benchmark | docs/manual-qa.md |
| Public website deployment | Public web | Deploy production site | Public website URL |

## Execution Order

| Order | Track | Blockers | Exit condition | Record target |
|---:|---|---|---|---|
| 1 | Local packaged-app proof | Packaged macOS manual QA, Benchmark release set | done | docs |
| 2 | Public web proof | Public website deployment | done | url |
",
    )
    .unwrap();

    assert_eq!(report.total, 3);
    assert_eq!(report.verified, 1);
    assert_eq!(report.blocked, 2);
    assert_eq!(report.tracks[0].remaining, vec!["Benchmark release set"]);
    let text = render::text(&report, None, &Scope::default()).unwrap();
    assert!(text.contains("release blockers: 3 total, 1 verified, 2 blocked"));
    assert!(text.contains("1. Local packaged-app proof: 1/2 remaining"));
    assert!(text.contains("remaining blockers: Benchmark release set"));
    assert!(text.contains("record target: docs"));
    assert!(text.contains("next actions: Benchmark release set -> docs/manual-qa.md"));
    assert!(text.contains("primary command: cargo run -p xtask -- manual-qa-pending"));
    assert!(text.contains("next track: 1. Local packaged-app proof"));
    assert!(text.contains("- Benchmark release set: Run release-set benchmark (docs/manual-qa.md)"));
}

#[test]
fn reports_complete_when_all_tracks_are_verified() {
    let report = parse::report(
        "\
| Blocker | Status | Completion evidence | Evidence reference | Record in |
|---|---|---|---|---|
| Signed DMG | Verified | done | Release notes | Release notes |

## Execution Order

| Order | Track | Blockers | Exit condition | Record target |
|---:|---|---|---|---|
| 4 | Signing and distribution proof | Signed DMG | done | Release notes |
",
    )
    .unwrap();

    assert!(render::text(&report, None, &Scope::default())
        .unwrap()
        .contains("next track: complete"));
}

#[test]
fn filters_to_requested_track() {
    let report = parse::report(
        "\
| Blocker | Status | Completion evidence | Evidence reference | Record in |
|---|---|---|---|---|
| Benchmark release set | Blocked | evidence | TBD | docs |
| Public website deployment | Blocked | evidence | TBD | url |

## Evidence Classes

| Blocker | Class | Next action | Evidence owner |
|---|---|---|---|
| Benchmark release set | Benchmark | Run release-set benchmark | docs/manual-qa.md |
| Public website deployment | Public web | Deploy production site | Public website URL |

## Execution Order

| Order | Track | Blockers | Exit condition | Record target |
|---:|---|---|---|---|
| 1 | Local packaged-app proof | Benchmark release set | done | docs |
| 3 | Public web proof | Public website deployment | done | url |
",
    )
    .unwrap();

    let text = render::text(&report, Some("3"), &Scope::default()).unwrap();

    assert!(text.contains("3. Public web proof: 1/1 remaining"));
    assert!(text.contains("cargo run -p xtask -- public-web-ready"));
    assert!(!text.contains("1. Local packaged-app proof"));
}

#[test]
fn completed_filtered_track_reports_complete() {
    let report = parse::report(
        "\
| Blocker | Status | Completion evidence | Evidence reference | Record in |
|---|---|---|---|---|
| Packaged macOS manual QA | Verified | done | docs/manual-qa.md | docs/manual-qa.md |

## Execution Order

| Order | Track | Blockers | Exit condition | Record target |
|---:|---|---|---|---|
| 1 | Local packaged-app proof | Packaged macOS manual QA | done | docs/manual-qa.md |
",
    )
    .unwrap();

    let text = render::text(&report, Some("Local packaged-app proof"), &Scope::default()).unwrap();

    assert!(text.contains("1. Local packaged-app proof: 0/1 remaining"));
    assert!(text.contains("next track: complete"));
}

#[test]
fn local_packaged_track_uses_packaged_rerun_after_benchmark_is_verified() {
    let report = parse::report(
        "\
| Blocker | Status | Completion evidence | Evidence reference | Record in |
|---|---|---|---|---|
| Packaged macOS manual QA | Blocked | evidence | TBD | docs/manual-qa.md |
| Benchmark release set | Verified | evidence | docs/manual-qa.md | docs/manual-qa.md |

## Evidence Classes

| Blocker | Class | Next action | Evidence owner |
|---|---|---|---|
| Packaged macOS manual QA | Manual packaged-app | Run the public DMG manual QA | docs/manual-qa.md |

## Execution Order

| Order | Track | Blockers | Exit condition | Record target |
|---:|---|---|---|---|
| 1 | Local packaged-app proof | Packaged macOS manual QA, Benchmark release set | done | docs/manual-qa.md |
",
    )
    .unwrap();

    let text = render::text(&report, None, &Scope::default()).unwrap();

    assert!(text.contains("1. Local packaged-app proof: 1/2 remaining"));
    assert!(text.contains("primary command: cargo run -p xtask -- manual-qa-packaged-rerun"));
    assert!(text.contains("start with cargo run -p xtask -- manual-qa-packaged-rerun"));
    assert!(text.contains("then: confirm relaunch returns focus to the existing mounted-DMG window without increasing the mounted app pid count"));
}

#[test]
fn next_actions_point_to_fast_paths_for_license_and_distribution() {
    let report = parse::report(
        "\
| Blocker | Status | Completion evidence | Evidence reference | Record in |
|---|---|---|---|---|
| Lemon Squeezy product setup | Blocked | evidence | TBD | docs/manual-qa.md |
| Signed DMG | Blocked | evidence | TBD | Release notes |

## Evidence Classes

| Blocker | Class | Next action | Evidence owner |
|---|---|---|---|
| Lemon Squeezy product setup | License sandbox | Record sandbox setup | docs/manual-qa.md |
| Signed DMG | Signing/notarization | Sign the public DMG | Release notes |

## Execution Order

| Order | Track | Blockers | Exit condition | Record target |
|---:|---|---|---|---|
| 2 | License sandbox proof | Lemon Squeezy product setup | done | docs/manual-qa.md |
| 4 | Signing and distribution proof | Signed DMG | done | Release notes |
",
    )
    .unwrap();

    let license_text = render::text(&report, None, &Scope::default()).unwrap();
    let distribution_text = render::text(&report, Some("4"), &Scope::default()).unwrap();

    assert!(license_text.contains(
        "manual-qa-license-rerun; see docs/license-sandbox-runbook.md `Short Execution Memo`"
    ));
    assert!(distribution_text.contains(
        "manual-qa-distribution-rerun; see docs/signed-dmg-runbook.md `Short Execution Memo`"
    ));
}

#[test]
fn rich_next_actions_do_not_repeat_fast_path_commands() {
    let report = parse::report(
        "\
| Blocker | Status | Completion evidence | Evidence reference | Record in |
|---|---|---|---|---|
| Lemon Squeezy product setup | Blocked | evidence | TBD | docs/manual-qa.md |
| Published checksum | Blocked | evidence | TBD | GitHub Release |

## Evidence Classes

| Blocker | Class | Next action | Evidence owner |
|---|---|---|---|
| Lemon Squeezy product setup | License sandbox | Start with `cargo run -p xtask -- manual-qa-license-rerun`; see docs/license-sandbox-runbook.md `Short Execution Memo` for the fast path | docs/manual-qa.md |
| Published checksum | Distribution | Attach SHA256SUMS, then start with `cargo run -p xtask -- publish-check path/to/release-notes.md`; see docs/public-beta-operator-checklist.md `Short Execution Memo` for the fast path | GitHub Release URL |

## Execution Order

| Order | Track | Blockers | Exit condition | Record target |
|---:|---|---|---|---|
| 2 | License sandbox proof | Lemon Squeezy product setup | done | docs/manual-qa.md |
| 4 | Signing and distribution proof | Published checksum | done | GitHub Release URL |
",
    )
    .unwrap();

    let license_text = render::text(&report, Some("2"), &Scope::default()).unwrap();
    let distribution_text = render::text(&report, Some("4"), &Scope::default()).unwrap();

    assert_eq!(license_text.matches("manual-qa-license-rerun").count(), 2);
    assert_eq!(distribution_text.matches("publish-check").count(), 1);
}

#[test]
fn parses_track_flag_without_path() {
    let (path, track) = parse_args(vec!["--track".into(), "3".into()]).unwrap();
    assert_eq!(path, std::path::PathBuf::from("docs/release-blockers.md"));
    assert_eq!(track.as_deref(), Some("3"));
}

#[test]
fn treats_paid_beta_alias_as_scoped_view() {
    assert!(is_paid_beta_alias("Paid beta"));
    assert!(is_paid_beta_alias("paid-beta"));
    assert_eq!(normalize_track(Some("Paid beta".into())), None);
    assert_eq!(
        normalize_track(Some("License sandbox proof".into())),
        Some("License sandbox proof".into())
    );
}

#[test]
fn omits_deferred_public_web_track_from_default_paid_beta_view() {
    let report = parse::report(
        "\
| Blocker | Status | Completion evidence | Evidence reference | Record in |
|---|---|---|---|---|
| Public website deployment | Blocked | evidence | TBD | url |
| Signed DMG | Blocked | evidence | TBD | release |

## Evidence Classes

| Blocker | Class | Next action | Evidence owner |
|---|---|---|---|
| Public website deployment | Public web | Deploy production site | Public website URL |
| Signed DMG | Signing/notarization | Sign dmg | Release notes |

## Execution Order

| Order | Track | Blockers | Exit condition | Record target |
|---:|---|---|---|---|
| 3 | Public web proof | Public website deployment | done | url |
| 4 | Signing and distribution proof | Signed DMG | done | release |
",
    )
    .unwrap();

    let scope = Scope::from_deferred(vec!["Public website deployment"]);
    let text = render::text(&report, None, &scope).unwrap();

    assert!(text.contains("paid beta now: 1 total, 0 verified, 1 blocked"));
    assert!(text.contains("deferred from paid-beta technical proof: Public website deployment"));
    assert!(text.contains("paid beta proof map: docs/paid-beta-readiness.md"));
    assert!(text.contains("paid beta operator checklist: docs/paid-beta-operator-checklist.md"));
    assert!(text
        .contains("paid beta manual QA helper: cargo run -p xtask -- manual-qa-paid-beta-rerun"));
    assert!(text.contains(
        "after paid beta manual QA helper: use `paid beta license markdown rows` and `paid beta distribution markdown rows` before section gates when only the remaining copy-ready rows are needed"
    ));
    assert!(text.contains(
        "paid beta deterministic helper: if you have a fresh prepared manual-QA draft and checked benchmark CSV, run cargo run -p xtask -- manual-qa-ready-all <manual-qa.md> <results.csv> before section reruns"
    ));
    assert!(text.contains(
        "paid beta direct signing track: cargo run -p xtask -- productization-status --track \"Signing and distribution proof\""
    ));
    assert!(text.contains(
        "these deferred items still remain required before production payment onboarding or a public paid beta"
    ));
    assert!(text.contains(
        "when paid-beta technical proof is complete, continue with: docs/public-beta-operator-checklist.md"
    ));
    assert!(text.contains(
        "deferred track detail: cargo run -p xtask -- productization-status --track \"Public web proof\""
    ));
    assert!(!text.contains("3. Public web proof"));
    assert!(text.contains("4. Signing and distribution proof: 1/1 remaining"));
    assert!(!text.contains(
        "paid beta direct license track: cargo run -p xtask -- productization-status --track \"License sandbox proof\""
    ));
}
