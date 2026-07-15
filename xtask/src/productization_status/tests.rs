use super::{parse, parse_args, render};

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
    let text = render::text(&report, None).unwrap();
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

    assert!(render::text(&report, None).unwrap().contains("next track: complete"));
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

    let text = render::text(&report, Some("3")).unwrap();

    assert!(text.contains("3. Public web proof: 1/1 remaining"));
    assert!(!text.contains("1. Local packaged-app proof"));
}

#[test]
fn parses_track_flag_without_path() {
    let (path, track) = parse_args(vec!["--track".into(), "3".into()]).unwrap();
    assert_eq!(path, std::path::PathBuf::from("docs/release-blockers.md"));
    assert_eq!(track.as_deref(), Some("3"));
}
