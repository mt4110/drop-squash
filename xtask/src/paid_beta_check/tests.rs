use super::{parse, run};

#[test]
fn parses_required_paid_beta_blockers() {
    let items = parse::required_blockers(
        "\
## Required Evidence

- Packaged macOS manual QA
- Valid sandbox activation

These rows are already supporting that path and should stay verified:
",
    )
    .unwrap();

    assert_eq!(
        items,
        ["Packaged macOS manual QA", "Valid sandbox activation"]
    );
}

#[test]
fn ignores_operator_checklist_bullets_after_required_blockers() {
    let items = parse::required_blockers(
        "\
## Required Evidence

- Packaged macOS manual QA
- Valid sandbox activation

Quick status:

```sh
cargo run -p xtask -- paid-beta-check
```

Operator checklist:

- [docs/paid-beta-operator-checklist.md](/tmp/docs/paid-beta-operator-checklist.md)

These rows are already supporting that path and should stay verified:
",
    )
    .unwrap();

    assert_eq!(
        items,
        ["Packaged macOS manual QA", "Valid sandbox activation"]
    );
}

#[test]
fn parses_release_blocker_statuses() {
    let statuses = parse::blocker_statuses(
        "\
| Blocker | Status | Completion evidence | Evidence reference | Record in |
|---|---|---|---|---|
| Packaged macOS manual QA | Blocked | x | y | z |
| Valid sandbox activation | Verified | x | y | z |
",
    );

    assert_eq!(statuses.get("Packaged macOS manual QA"), Some(&"Blocked"));
    assert_eq!(statuses.get("Valid sandbox activation"), Some(&"Verified"));
}

#[test]
fn fails_when_required_paid_beta_blockers_remain() {
    let dir = tempfile::tempdir().unwrap();
    let blockers = dir.path().join("release-blockers.md");
    let readiness = dir.path().join("paid-beta-readiness.md");
    std::fs::write(
        &blockers,
        "\
| Blocker | Status | Completion evidence | Evidence reference | Record in |
|---|---|---|---|---|
| Packaged macOS manual QA | Blocked | x | y | z |
| Valid sandbox activation | Verified | x | y | z |
",
    )
    .unwrap();
    std::fs::write(
        &readiness,
        "\
## Required Evidence

- Packaged macOS manual QA
- Valid sandbox activation

These rows are already supporting that path and should stay verified:
",
    )
    .unwrap();

    let error = run(vec![
        blockers.display().to_string(),
        readiness.display().to_string(),
    ])
    .unwrap_err();

    assert!(error.contains("1 remaining"));
    assert!(error.contains("Packaged macOS manual QA: Blocked"));
    assert!(error.contains("private/manual paid beta technical proof"));
    assert!(error.contains("cargo run -p xtask -- public-web-ready"));
    assert!(error.contains("cargo run -p xtask -- public-web-rerun"));
    assert!(error.contains("productization-status --track 'Public web proof'"));
    assert!(error.contains("docs/public-beta-operator-checklist.md"));
    assert!(error.contains("Short Execution Memo"));
    assert!(error.contains("next commands:"));
    assert!(error.contains("paid beta operator checklist: docs/paid-beta-operator-checklist.md"));
    assert!(error.contains("after manual-qa-paid-beta-rerun: use `paid beta license markdown rows` and `paid beta distribution markdown rows` before the section gates"));
    assert!(error.contains("packaged manual QA guide: docs/manual-qa.md"));
    assert!(error.contains("proof map: Packaged macOS manual QA -> manual-qa-packaged-rerun"));
    assert!(error.contains("manual-qa-paid-beta-rerun"));
    assert!(error.contains("manual-qa-pending"));
    assert!(error.contains("manual-qa-ready-local-proof"));
    assert!(error.contains("packaged observation reminder: confirm relaunch returns focus to the existing mounted-DMG window without increasing the mounted app pid count"));
}

#[test]
fn succeeds_when_required_paid_beta_blockers_are_verified() {
    let dir = tempfile::tempdir().unwrap();
    let blockers = dir.path().join("release-blockers.md");
    let readiness = dir.path().join("paid-beta-readiness.md");
    std::fs::write(
        &blockers,
        "\
| Blocker | Status | Completion evidence | Evidence reference | Record in |
|---|---|---|---|---|
| Packaged macOS manual QA | Verified | x | y | z |
| Valid sandbox activation | Verified | x | y | z |
",
    )
    .unwrap();
    std::fs::write(
        &readiness,
        "\
## Required Evidence

- Packaged macOS manual QA
- Valid sandbox activation

These rows are already supporting that path and should stay verified:
",
    )
    .unwrap();

    let output = run(vec![
        blockers.display().to_string(),
        readiness.display().to_string(),
    ]);

    assert!(output.is_ok());
}

#[test]
fn suggests_license_and_distribution_commands_for_matching_blockers() {
    let dir = tempfile::tempdir().unwrap();
    let blockers = dir.path().join("release-blockers.md");
    let readiness = dir.path().join("paid-beta-readiness.md");
    let manual = dir.path().join("manual-qa.md");
    let csv = dir.path().join("results.csv");
    std::fs::write(&csv, "backend,input,output\n").unwrap();
    std::fs::write(
        &blockers,
        "\
| Blocker | Status | Completion evidence | Evidence reference | Record in |
|---|---|---|---|---|
| Lemon Squeezy product setup | Blocked | x | y | z |
| Signed DMG | Blocked | x | y | z |
",
    )
    .unwrap();
    std::fs::write(
        &readiness,
        "\
## Required Evidence

- Lemon Squeezy product setup
- Signed DMG

These rows are already supporting that path and should stay verified:
",
    )
    .unwrap();
    std::fs::write(
        &manual,
        format!(
            "| Check | Expected | Result |\n|---|---|---|\n| Benchmark sample set | Three samples | CSV saved outside repo at {} |\n",
            csv.display()
        ),
    )
    .unwrap();

    let error = run(vec![
        blockers.display().to_string(),
        readiness.display().to_string(),
        manual.display().to_string(),
    ])
    .unwrap_err();

    assert!(error
        .contains("preferred deterministic first pass: cargo run -p xtask -- manual-qa-ready-all"));
    assert!(error.contains("manual-qa-prepare --reset-trial"));
    assert!(error.contains("paid beta operator checklist: docs/paid-beta-operator-checklist.md"));
    assert!(error.contains("direct signing track: cargo run -p xtask -- productization-status --track \"Signing and distribution proof\""));
    assert!(error.contains("direct license track: cargo run -p xtask -- productization-status --track \"License sandbox proof\""));
    assert!(error.contains("license sandbox runbook: docs/license-sandbox-runbook.md"));
    assert!(error.contains("manual beta license issuance: docs/manual-beta-license-issuance.md"));
    assert!(error.contains("license browser sign-in checkpoint"));
    assert!(error.contains("Sign in to Lemon Squeezy"));
    assert!(error.contains("auth.lemonsqueezy.com/login"));
    assert!(error.contains("license browser sign-in success"));
    assert!(error.contains("dashboard is open"));
    assert!(error.contains("sandbox mode is visible"));
    assert!(error.contains("Short Execution Memo"));
    assert!(error.contains(
        "proof map: Lemon Squeezy product setup / sandbox purchase / valid sandbox activation"
    ));
    assert!(error.contains("manual-qa-license-rerun"));
    assert!(error.contains("next sandbox markdown rows command"));
    assert!(error.contains("next sandbox row candidates"));
    assert!(error.contains("sandbox quickstart 1..4"));
    assert!(error.contains("license activation loop"));
    assert!(error.contains("sandbox quickstart 2 before the UI action"));
    assert!(error.contains("sandbox quickstart 4 after Pro appears"));
    assert!(error.contains("--section license"));
    assert!(error.contains("license sandbox step 1"));
    assert!(error.contains("license sandbox step 2"));
    assert!(error.contains("license sandbox step 3"));
    assert!(error.contains("signed DMG runbook: docs/signed-dmg-runbook.md"));
    assert!(error.contains("distribution signing environment note: local macOS needs APPLE_SIGNING_IDENTITY or APPLE_CERTIFICATE with APPLE_CERTIFICATE_PASSWORD; notarization needs APPLE_API_KEY/APPLE_API_ISSUER/APPLE_API_KEY_PATH or APPLE_ID/APPLE_PASSWORD/APPLE_TEAM_ID; CI signing also needs APPLE_KEYCHAIN_PASSWORD and APPLE_CODESIGN_IDENTITY"));
    assert!(error.contains(
        "proof map: Signed DMG / notarized and stapled DMG / Gatekeeper clean-machine open"
    ));
    assert!(error.contains("manual-qa-distribution-rerun"));
    assert!(error
        .contains("manual-qa-pending --section distribution | rg 'distribution .*markdown row:'"));
    assert!(error.contains("distribution quickstart 1..7"));
    assert!(error.contains("--section distribution"));
}

#[test]
fn reports_pending_manual_qa_rows() {
    let dir = tempfile::tempdir().unwrap();
    let blockers = dir.path().join("release-blockers.md");
    let readiness = dir.path().join("paid-beta-readiness.md");
    let manual = dir.path().join("manual-qa.md");
    std::fs::write(
        &blockers,
        "\
| Blocker | Status | Completion evidence | Evidence reference | Record in |
|---|---|---|---|---|
| Lemon Squeezy product setup | Blocked | x | y | z |
",
    )
    .unwrap();
    std::fs::write(
        &readiness,
        "\
## Required Evidence

- Lemon Squeezy product setup

These rows are already supporting that path and should stay verified:
",
    )
    .unwrap();
    std::fs::write(
        &manual,
        "\
| Check | Expected | Result |
|---|---|---|
| Sandbox product setup | x |  |
| Codesign verification | x | done |
",
    )
    .unwrap();

    let error = run(vec![
        blockers.display().to_string(),
        readiness.display().to_string(),
        manual.display().to_string(),
    ])
    .unwrap_err();

    assert!(error.contains("manual QA local proof rows: none pending"));
    assert!(error.contains("manual QA license rows still empty:"));
    assert!(error.contains("- Sandbox product setup"));
    assert!(error.contains("manual QA distribution rows: none pending"));
}

#[test]
fn clarifies_distribution_rows_include_helper_lines() {
    let dir = tempfile::tempdir().unwrap();
    let blockers = dir.path().join("release-blockers.md");
    let readiness = dir.path().join("paid-beta-readiness.md");
    let manual = dir.path().join("manual-qa.md");
    std::fs::write(
        &blockers,
        "\
| Blocker | Status | Completion evidence | Evidence reference | Record in |
|---|---|---|---|---|
| Signed DMG | Blocked | x | y | z |
",
    )
    .unwrap();
    std::fs::write(
        &readiness,
        "\
## Required Evidence

- Signed DMG

These rows are already supporting that path and should stay verified:
",
    )
    .unwrap();
    std::fs::write(
        &manual,
        "\
| Check | Expected | Result |
|---|---|---|
| `cargo run -p xtask -- macos-signing-check` | x |  |
| Codesign verification | x |  |
",
    )
    .unwrap();

    let error = run(vec![
        blockers.display().to_string(),
        readiness.display().to_string(),
        manual.display().to_string(),
    ])
    .unwrap_err();

    assert!(error.contains("manual QA distribution rows still empty:"));
    assert!(error.contains("- `cargo run -p xtask -- macos-signing-check`"));
    assert!(error.contains("- Codesign verification"));
    assert!(error.contains("manual QA distribution note: these pending rows include helper/preflight rows plus the three paid-beta blocker rows Signed DMG, Notarized and stapled DMG, and Gatekeeper clean-machine open"));
}

#[test]
fn suggests_combined_paid_beta_rerun_for_license_only_blockers() {
    let dir = tempfile::tempdir().unwrap();
    let blockers = dir.path().join("release-blockers.md");
    let readiness = dir.path().join("paid-beta-readiness.md");
    std::fs::write(
        &blockers,
        "\
| Blocker | Status | Completion evidence | Evidence reference | Record in |
|---|---|---|---|---|
| Lemon Squeezy product setup | Blocked | x | y | z |
",
    )
    .unwrap();
    std::fs::write(
        &readiness,
        "\
## Required Evidence

- Lemon Squeezy product setup

These rows are already supporting that path and should stay verified:
",
    )
    .unwrap();

    let error = run(vec![
        blockers.display().to_string(),
        readiness.display().to_string(),
    ])
    .unwrap_err();

    assert!(error.contains("manual-qa-paid-beta-rerun"));
    assert!(error.contains("direct license track: cargo run -p xtask -- productization-status --track \"License sandbox proof\""));
    assert!(error.contains("license sandbox runbook: docs/license-sandbox-runbook.md"));
    assert!(error.contains(
        "proof map: Lemon Squeezy product setup / sandbox purchase / valid sandbox activation"
    ));
    assert!(error.contains("manual-qa-license-rerun"));
}

#[test]
fn suggests_deterministic_recovery_when_benchmark_csv_is_missing() {
    let dir = tempfile::tempdir().unwrap();
    let blockers = dir.path().join("release-blockers.md");
    let readiness = dir.path().join("paid-beta-readiness.md");
    std::fs::write(
        &blockers,
        "\
| Blocker | Status | Completion evidence | Evidence reference | Record in |
|---|---|---|---|---|
| Lemon Squeezy product setup | Blocked | x | y | z |
| Signed DMG | Blocked | x | y | z |
",
    )
    .unwrap();
    std::fs::write(
        &readiness,
        "\
## Required Evidence

- Lemon Squeezy product setup
- Signed DMG

These rows are already supporting that path and should stay verified:
",
    )
    .unwrap();

    let error = run(vec![
        blockers.display().to_string(),
        readiness.display().to_string(),
    ])
    .unwrap_err();

    assert!(error.contains("preferred deterministic recovery: rerun `manual-qa-prepare --reset-trial`, `benchmark --release-set`, `benchmark-csv-check`, then `manual-qa-ready-all` before sandbox or signing reruns"));
}

#[test]
fn explains_when_packaged_manual_qa_is_still_blocked_after_local_proof() {
    let dir = tempfile::tempdir().unwrap();
    let blockers = dir.path().join("release-blockers.md");
    let readiness = dir.path().join("paid-beta-readiness.md");
    let manual = dir.path().join("manual-qa.md");
    std::fs::write(
        &blockers,
        "\
| Blocker | Status | Completion evidence | Evidence reference | Record in |
|---|---|---|---|---|
| Packaged macOS manual QA | Blocked | x | y | z |
",
    )
    .unwrap();
    std::fs::write(
        &readiness,
        "\
## Required Evidence

- Packaged macOS manual QA

These rows are already supporting that path and should stay verified:
",
    )
    .unwrap();
    std::fs::write(
        &manual,
        "\
| Check | Expected | Result |
|---|---|---|
| Codesign verification | x | done |
",
    )
    .unwrap();

    let error = run(vec![
        blockers.display().to_string(),
        readiness.display().to_string(),
        manual.display().to_string(),
    ])
    .unwrap_err();

    assert!(error.contains("manual QA local proof rows: none pending"));
    assert!(error.contains("manual QA packaged-app note: local-proof rows are complete"));
    assert!(error.contains("manual-qa.md is filled against the tested public DropSquash.dmg"));
    assert!(error.contains("packaged manual QA guide: docs/manual-qa.md"));
    assert!(error.contains("run `manual-qa-ready-all` as the standard deterministic entrypoint"));
    assert!(error.contains("`manual-qa-ready-local-proof` as the packaged-only alternative"));
    assert!(error.contains("packaged observation reminder: confirm relaunch returns focus to the existing mounted-DMG window without increasing the mounted app pid count"));
}

#[test]
fn packaged_manual_qa_note_uses_passed_manual_path() {
    let dir = tempfile::tempdir().unwrap();
    let blockers = dir.path().join("release-blockers.md");
    let readiness = dir.path().join("paid-beta-readiness.md");
    let manual = dir.path().join("Snapshot Manual QA.md");
    std::fs::write(
        &blockers,
        "\
| Blocker | Status | Completion evidence | Evidence reference | Record in |
|---|---|---|---|---|
| Packaged macOS manual QA | Blocked | x | y | z |
",
    )
    .unwrap();
    std::fs::write(
        &readiness,
        "\
## Required Evidence

- Packaged macOS manual QA

These rows are already supporting that path and should stay verified:
",
    )
    .unwrap();
    std::fs::write(
        &manual,
        "\
| Check | Expected | Result |
|---|---|---|
| Codesign verification | x | done |
",
    )
    .unwrap();

    let error = run(vec![
        blockers.display().to_string(),
        readiness.display().to_string(),
        manual.display().to_string(),
    ])
    .unwrap_err();

    assert!(
        error.contains("Snapshot Manual QA.md is filled against the tested public DropSquash.dmg")
    );
}

#[test]
fn packaged_manual_qa_note_shows_known_benchmark_csv() {
    let dir = tempfile::tempdir().unwrap();
    let blockers = dir.path().join("release-blockers.md");
    let readiness = dir.path().join("paid-beta-readiness.md");
    let manual = dir.path().join("manual-qa.md");
    let csv = dir.path().join("results.csv");
    std::fs::write(&csv, "backend,input,output\n").unwrap();
    std::fs::write(
        &blockers,
        "\
| Blocker | Status | Completion evidence | Evidence reference | Record in |
|---|---|---|---|---|
| Packaged macOS manual QA | Blocked | x | y | z |
",
    )
    .unwrap();
    std::fs::write(
        &readiness,
        "\
## Required Evidence

- Packaged macOS manual QA

These rows are already supporting that path and should stay verified:
",
    )
    .unwrap();
    std::fs::write(
        &manual,
        format!(
            "| Check | Expected | Result |\n|---|---|---|\n| Benchmark sample set | Three samples | CSV saved outside repo at {} |\n",
            csv.display()
        ),
    )
    .unwrap();

    let error = run(vec![
        blockers.display().to_string(),
        readiness.display().to_string(),
        manual.display().to_string(),
    ])
    .unwrap_err();

    assert!(error.contains("manual QA packaged-app benchmark CSV:"));
    assert!(error.contains(&csv.display().to_string()));
}

#[test]
fn packaged_manual_qa_note_flags_stale_app_build_and_local_artifact() {
    let dir = tempfile::tempdir().unwrap();
    let blockers = dir.path().join("release-blockers.md");
    let readiness = dir.path().join("paid-beta-readiness.md");
    let manual = dir.path().join("manual-qa.md");
    std::fs::write(
        &blockers,
        "\
| Blocker | Status | Completion evidence | Evidence reference | Record in |
|---|---|---|---|---|
| Packaged macOS manual QA | Blocked | x | y | z |
",
    )
    .unwrap();
    std::fs::write(
        &readiness,
        "\
## Required Evidence

- Packaged macOS manual QA

These rows are already supporting that path and should stay verified:
",
    )
    .unwrap();
    std::fs::write(
        &manual,
        "\
| App build | DropSquash 0.1.0 git 0000000 |
| App artifact | /tmp/work/target/release/bundle/dmg/DropSquash.dmg |
| Check | Expected | Result |
|---|---|---|
| Codesign verification | x | done |
",
    )
    .unwrap();

    let error = run(vec![
        blockers.display().to_string(),
        readiness.display().to_string(),
        manual.display().to_string(),
    ])
    .unwrap_err();

    assert!(error.contains("manual QA App build note: recorded App build"));
    assert!(error.contains("does not match the current HEAD"));
    assert!(error.contains(
        "manual QA App artifact note: recorded App artifact still points at a local build output"
    ));
}

#[test]
fn suggests_public_artifact_follow_up_when_packaged_rows_are_already_filled() {
    let dir = tempfile::tempdir().unwrap();
    let blockers = dir.path().join("release-blockers.md");
    let readiness = dir.path().join("paid-beta-readiness.md");
    let manual = dir.path().join("manual-qa.md");
    let csv = dir.path().join("results.csv");
    std::fs::write(&csv, "backend,input,output\n").unwrap();
    std::fs::write(
        &blockers,
        "\
| Blocker | Status | Completion evidence | Evidence reference | Record in |
|---|---|---|---|---|
| Packaged macOS manual QA | Blocked | x | y | z |
",
    )
    .unwrap();
    std::fs::write(
        &readiness,
        "\
## Required Evidence

- Packaged macOS manual QA

These rows are already supporting that path and should stay verified:
",
    )
    .unwrap();
    std::fs::write(
        &manual,
        format!(
            "| Check | Expected | Result |\n|---|---|---|\n| Codesign verification | x | done |\n| Benchmark sample set | Three samples | CSV saved outside repo at {} |\n",
            csv.display()
        ),
    )
    .unwrap();

    let error = run(vec![
        blockers.display().to_string(),
        readiness.display().to_string(),
        manual.display().to_string(),
    ])
    .unwrap_err();

    assert!(error.contains("cargo run -p xtask -- manual-qa-check"));
    assert!(error.contains("after the public DropSquash.dmg exists"));
    assert!(error.contains("change `Packaged macOS manual QA` from `Blocked` to `Verified`"));
    assert!(error.contains("sample-link and installed-app helper commands"));
    assert!(error.contains("manual-qa-ready-local-proof"));
}

#[test]
fn uses_passed_manual_qa_path_in_suggested_commands() {
    let dir = tempfile::tempdir().unwrap();
    let blockers = dir.path().join("release-blockers.md");
    let readiness = dir.path().join("paid-beta-readiness.md");
    let manual = dir.path().join("My Manual QA.md");
    std::fs::write(
        &blockers,
        "\
| Blocker | Status | Completion evidence | Evidence reference | Record in |
|---|---|---|---|---|
| Signed DMG | Blocked | x | y | z |
",
    )
    .unwrap();
    std::fs::write(
        &readiness,
        "\
## Required Evidence

- Signed DMG

These rows are already supporting that path and should stay verified:
",
    )
    .unwrap();
    std::fs::write(&manual, "| Check | Expected | Result |\n|---|---|---|\n").unwrap();

    let error = run(vec![
        blockers.display().to_string(),
        readiness.display().to_string(),
        manual.display().to_string(),
    ])
    .unwrap_err();

    assert!(error.contains("manual-qa-distribution-rerun"));
    assert!(error.contains("distribution quickstart 1..7"));
    assert!(error.contains("manual-qa-pending"));
    assert!(error.contains("My Manual QA.md"));
    assert!(error.contains("My Manual QA.md"));
    assert!(error.contains("manual-qa-prepare --reset-trial"));
}

#[test]
fn suggests_local_proof_gate_when_packaged_blocker_remains_after_rows_are_filled() {
    let dir = tempfile::tempdir().unwrap();
    let blockers = dir.path().join("release-blockers.md");
    let readiness = dir.path().join("paid-beta-readiness.md");
    let manual = dir.path().join("manual-qa.md");
    std::fs::write(
        &blockers,
        "\
| Blocker | Status | Completion evidence | Evidence reference | Record in |
|---|---|---|---|---|
| Packaged macOS manual QA | Blocked | x | y | z |
| Signed DMG | Blocked | x | y | z |
",
    )
    .unwrap();
    std::fs::write(
        &readiness,
        "\
## Required Evidence

- Packaged macOS manual QA
- Signed DMG

These rows are already supporting that path and should stay verified:
",
    )
    .unwrap();
    std::fs::write(
        &manual,
        "\
| Check | Expected | Result |
|---|---|---|
| Codesign verification | x |  |
",
    )
    .unwrap();

    let error = run(vec![
        blockers.display().to_string(),
        readiness.display().to_string(),
        manual.display().to_string(),
    ])
    .unwrap_err();

    assert!(error.contains("manual QA local proof rows: none pending"));
    assert!(error.contains("manual-qa-check '"));
    assert!(error.contains("--section local-proof"));
}
