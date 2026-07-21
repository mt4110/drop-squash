use super::{
    distribution_lines, license_sandbox_lines, lines_for, manual_qa_lines, public_web_lines,
    summary,
};
use crate::productization_status::model::TrackStatus;
use crate::productization_status::scope::Scope;

#[test]
fn reports_clean_manual_qa_preflight() {
    let lines = manual_qa_lines("");
    assert!(lines[0].contains("can start"));
    assert!(lines[1].contains("apps/desktop install --frozen-lockfile"));
    assert!(lines[2].contains("apps/desktop/web install --frozen-lockfile"));
    assert!(lines[3].contains("tauri build"));
    assert!(lines[7].contains("markdown-output note"));
    assert!(lines[11].contains("manual-qa-ready-all"));
    assert!(lines[12].contains("manual-qa-ready-local-proof"));
    assert!(lines[13].contains("--section local-proof"));
    assert!(lines[15].contains("--restore-state"));
}

#[test]
fn reports_dirty_manual_qa_preflight() {
    let lines = manual_qa_lines(" M docs/manual-qa.md\n");
    assert!(lines[0].contains("current worktree is dirty"));
    assert_eq!(lines[1], " M docs/manual-qa.md");
    assert!(lines[3].contains("git worktree add --detach"));
    assert!(lines[3].contains("manual-qa-snapshot-worktree.sh /tmp/dropsquash-qa-snapshot-"));
    assert!(lines[9].contains("manual-qa-prepare"));
    assert!(lines[10].contains("markdown-output note"));
    assert!(lines[14].contains("manual-qa-ready-all"));
    assert!(lines[15].contains("manual-qa-ready-local-proof"));
    assert!(lines[18].contains("--restore-state"));
}

#[test]
fn reports_license_sandbox_preflight() {
    let lines = license_sandbox_lines();
    assert!(lines[0].contains("sandbox credentials"));
    assert!(lines[1].contains("manual-qa-prepare"));
    assert!(lines[2].contains("preflight deterministic helper:"));
    assert!(lines[2].contains("manual-qa-ready-all"));
    assert!(lines[3].contains("manual-qa-paid-beta-rerun"));
    assert!(lines[4].contains("manual-qa-license-rerun"));
    assert!(lines[5].contains("manual-qa-pending"));
    assert!(lines[5].contains("sandbox purchase"));
    assert!(lines[6].contains("paid-beta-check"));
    assert!(lines[7].contains("license status"));
    assert!(lines[8].contains("manual-qa-check"));
}

#[test]
fn reports_public_web_preflight() {
    let lines = public_web_lines();
    assert!(lines[0].contains("production dropsquash.app URLs"));
    assert!(lines[1].contains("public-web-ready"));
    assert!(lines[2].contains("public-web-rerun"));
    assert!(lines[3].contains("website-check"));
    assert!(lines[4].contains("verify:site"));
    assert!(lines[5].contains("public-web-probe"));
    assert!(lines[6].contains("dig +short A dropsquash.app"));
    assert!(lines[7].contains("HTTP 200"));
    assert!(lines[8].contains("provenance"));
    assert!(lines[9].contains("pricing and refund are now aligned on production"));
    assert!(lines[10].contains(&crate::current_date::display()));
    assert!(lines[11].contains("resolver failures"));
    assert!(lines[12].contains("Live checkout link"));
    assert!(lines[13].contains("owner-history host"));
    assert!(lines[14].contains("public-beta-operator-checklist.md"));
    assert!(lines[15].contains("release-status"));
    assert!(lines[15].contains("store.lemonsqueezy.com/checkout/buy/"));
    assert!(lines[16].contains("publish-check"));
}

#[test]
fn reports_distribution_preflight() {
    let lines = distribution_lines();
    assert!(lines[0].contains("release environment secrets"));
    assert!(lines[1].contains("manual-qa-prepare"));
    assert!(lines[2].contains("preflight deterministic helper:"));
    assert!(lines[2].contains("manual-qa-ready-all"));
    assert!(lines[3].contains("manual-qa-paid-beta-rerun"));
    assert!(lines[4].contains("manual-qa-distribution-handoff.sh"));
    assert!(lines[5].contains("manual-qa-distribution-rerun"));
    assert!(lines[6].contains("manual-qa-pending"));
    assert!(lines[6].contains("distribution .*markdown row:"));
    assert!(lines[7].contains("paid-beta-check"));
    assert!(lines[8].contains("macos-signing-check"));
    assert!(lines[8].contains("CARGO_TARGET_DIR=/tmp/dsq-xtask-target"));
    assert!(lines[9].contains("APPLE_SIGNING_IDENTITY"));
    assert!(lines[9].contains("APPLE_API_KEY"));
    assert!(lines[10].contains("manual-qa-check"));
}

#[test]
fn reports_track_primary_command_summary() {
    let scope = Scope::default();
    let license = TrackStatus {
        order: 2,
        name: "License sandbox proof".into(),
        blockers: vec!["Lemon Squeezy product setup".into()],
        remaining: vec!["Lemon Squeezy product setup".into()],
        record_target: "docs/manual-qa.md".into(),
    };
    assert!(summary(&license, &scope, false)
        .unwrap()
        .contains("manual-qa-license-rerun"));
    let public_web = TrackStatus {
        order: 3,
        name: "Public web proof".into(),
        blockers: vec!["Public website deployment".into()],
        remaining: vec!["Public website deployment".into()],
        record_target: "https://dropsquash.app".into(),
    };
    assert!(summary(&public_web, &scope, false)
        .unwrap()
        .contains("public-web-ready"));
    assert!(summary(&public_web, &scope, false)
        .unwrap()
        .contains("publish-check remains the final public-release gate"));
}

#[test]
fn local_packaged_summary_switches_after_benchmark_is_done() {
    let scope = Scope::default();
    let benchmark_remaining = TrackStatus {
        order: 1,
        name: "Local packaged-app proof".into(),
        blockers: vec![
            "Packaged macOS manual QA".into(),
            "Benchmark release set".into(),
        ],
        remaining: vec!["Benchmark release set".into()],
        record_target: "docs/manual-qa.md".into(),
    };
    assert!(summary(&benchmark_remaining, &scope, false)
        .unwrap()
        .contains("manual-qa-pending"));
    let packaged_only = TrackStatus {
        order: 1,
        name: "Local packaged-app proof".into(),
        blockers: vec![
            "Packaged macOS manual QA".into(),
            "Benchmark release set".into(),
        ],
        remaining: vec!["Packaged macOS manual QA".into()],
        record_target: "docs/manual-qa.md".into(),
    };
    assert!(summary(&packaged_only, &scope, false)
        .unwrap()
        .contains("manual-qa-packaged-rerun"));
}

#[test]
fn filters_preflight_to_requested_track() {
    let report = crate::productization_status::parse::report(
        "\
| Blocker | Status | Completion evidence | Evidence reference | Record in |
|---|---|---|---|---|
| Public website deployment | Blocked | evidence | TBD | url |

## Evidence Classes

| Blocker | Class | Next action | Evidence owner |
|---|---|---|---|
| Public website deployment | Public web | Deploy production site | Public website URL |

## Execution Order

| Order | Track | Blockers | Exit condition | Record target |
|---:|---|---|---|---|
| 3 | Public web proof | Public website deployment | done | url |
",
    )
    .unwrap();

    let lines = lines_for(&report, Some("3"), &Scope::default()).unwrap();

    assert!(lines[0].contains("production dropsquash.app URLs"));
    assert!(lines[1].contains("public-web-ready"));
    assert!(lines[2].contains("public-web-rerun"));
    assert!(lines[3].contains("website-check"));
    assert!(lines[5].contains("public-web-probe"));
    assert!(lines[14].contains("public-beta-operator-checklist.md"));
    assert!(lines[16].contains("publish-check"));
}

#[test]
fn completed_requested_track_returns_no_preflight_instead_of_unknown() {
    let report = crate::productization_status::parse::report(
        "\
| Blocker | Status | Completion evidence | Evidence reference | Record in |
|---|---|---|---|---|
| Packaged macOS manual QA | Verified | evidence | docs/manual-qa.md | docs/manual-qa.md |

## Execution Order

| Order | Track | Blockers | Exit condition | Record target |
|---:|---|---|---|---|
| 1 | Local packaged-app proof | Packaged macOS manual QA | done | docs/manual-qa.md |
",
    )
    .unwrap();

    let lines = lines_for(&report, Some("Local packaged-app proof"), &Scope::default()).unwrap();

    assert!(lines.is_empty());
}
