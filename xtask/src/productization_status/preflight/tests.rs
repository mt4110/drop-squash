use super::{
    distribution_lines, license_sandbox_lines, lines_for, manual_qa_lines, public_web_lines,
    summary,
};

#[test]
fn reports_clean_manual_qa_preflight() {
    let lines = manual_qa_lines("");
    assert!(lines[0].contains("can start"));
    assert!(lines[1].contains("apps/desktop install --frozen-lockfile"));
    assert!(lines[2].contains("apps/desktop/web install --frozen-lockfile"));
    assert!(lines[3].contains("tauri build"));
    assert!(lines[10].contains("manual-qa-ready-local-proof"));
    assert!(lines[11].contains("--section local-proof"));
    assert!(lines[13].contains("--restore-state"));
}

#[test]
fn reports_dirty_manual_qa_preflight() {
    let lines = manual_qa_lines(" M docs/manual-qa.md\n");
    assert!(lines[0].contains("current worktree is dirty"));
    assert_eq!(lines[1], " M docs/manual-qa.md");
    assert!(lines[3].contains("git worktree add --detach"));
    assert!(lines[9].contains("manual-qa-prepare"));
    assert!(lines[16].contains("--restore-state"));
}

#[test]
fn reports_license_sandbox_preflight() {
    let lines = license_sandbox_lines();
    assert!(lines[0].contains("sandbox credentials"));
    assert!(lines[1].contains("manual-qa-ready-license"));
    assert!(lines[2].contains("license status"));
    assert!(lines[3].contains("manual-qa-check"));
}

#[test]
fn reports_public_web_preflight() {
    let lines = public_web_lines();
    assert!(lines[0].contains("production dropsquash.app URLs"));
    assert!(lines[1].contains("website-check"));
    assert!(lines[2].contains("publish-check"));
    assert!(lines[3].contains("release-status"));
    assert!(lines[3].contains("store.lemonsqueezy.com/checkout/buy/"));
}

#[test]
fn reports_distribution_preflight() {
    let lines = distribution_lines();
    assert!(lines[0].contains("release environment secrets"));
    assert!(lines[1].contains("manual-qa-ready-distribution"));
    assert!(lines[2].contains("macos-signing-check"));
    assert!(lines[3].contains("manual-qa-check"));
}

#[test]
fn reports_track_primary_command_summary() {
    assert!(
        summary("License sandbox proof")
            .unwrap()
            .contains("manual-qa-ready-license")
    );
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

    let lines = lines_for(&report, Some("3")).unwrap();

    assert!(lines[0].contains("production dropsquash.app URLs"));
    assert!(lines[1].contains("website-check"));
}
