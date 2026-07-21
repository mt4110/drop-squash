use std::path::Path;

use super::{lines, run};

#[test]
fn prints_packaged_rerun_commands() {
    let dir = tempfile::tempdir().unwrap();
    let csv = dir.path().join("results.csv");
    let manual = dir.path().join("manual.md");
    std::fs::write(&csv, "backend,input,output\n").unwrap();
    std::fs::write(
        &manual,
        format!(
            "| App artifact | /tmp/DropSquash.dmg |\n| Benchmark sample set | Three samples | CSV saved outside repo at {} |\n",
            csv.display()
        ),
    )
    .unwrap();

    run(vec![manual.display().to_string()]).unwrap();
}

#[test]
fn includes_visibility_observation_reminder_and_dmg_helpers() {
    let output = lines(Path::new("/tmp/manual.md"), "/tmp/results.csv", true);

    assert!(output.iter().any(|line| line.contains("docs/manual-qa.md")));
    assert!(output
        .iter()
        .any(|line| line.contains("docs/license-sandbox-runbook.md")));
    assert!(output
        .iter()
        .any(|line| line.contains("docs/signed-dmg-runbook.md")));
    assert!(output
        .iter()
        .any(|line| line.contains("scripts/manual-qa-distribution-handoff.sh")));
    assert!(output
        .iter()
        .any(|line| line.contains("docs/paid-beta-operator-checklist.md")));
    assert!(output
        .iter()
        .any(|line| line.contains("packaged observation reminder:")));
    assert!(output
        .iter()
        .any(|line| line.contains("mounted app pid count")));
    assert!(
        output
            .iter()
            .any(|line| line
                .contains("license field, and Choose recording action stay fully visible"))
    );
    assert!(output
        .iter()
        .any(|line| line.contains("manual-qa-installed-app stash")));
    assert!(output
        .iter()
        .any(|line| line.contains("manual-qa-window-probe")));
    assert!(
        output
            .iter()
            .any(|line| line
                .contains("paid beta blocker gate: cargo run -p xtask -- paid-beta-check"))
    );
}

#[test]
fn omits_dmg_helpers_for_non_dmg_artifact() {
    let output = lines(Path::new("/tmp/manual.md"), "/tmp/results.csv", false);

    assert!(!output
        .iter()
        .any(|line| line.contains("manual-qa-installed-app status")));
}

#[test]
fn errors_without_checked_csv() {
    let dir = tempfile::tempdir().unwrap();
    let manual = dir.path().join("manual.md");
    std::fs::write(
        &manual,
        "| Benchmark sample set | Three samples | no csv |\n",
    )
    .unwrap();

    let error = run(vec![manual.display().to_string()]).unwrap_err();

    assert!(error.contains("Benchmark sample set CSV"));
    assert!(error.contains("manual-qa-prepare --reset-trial"));
    assert!(error.contains("benchmark --release-set"));
    assert!(error.contains("benchmark-csv-check"));
    assert!(error.contains("manual-qa-ready-all"));
    assert!(error.contains("standard paid-beta pass"));
    assert!(error.contains("manual-qa-ready-local-proof"));
    assert!(error.contains("packaged-only proof"));
}
