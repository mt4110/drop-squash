use super::{next_lines, run};

#[test]
fn prints_all_paid_beta_rerun_lines() {
    let dir = tempfile::tempdir().unwrap();
    let csv = dir.path().join("results.csv");
    let manual = dir.path().join("manual.md");
    std::fs::write(&csv, "backend,input,output\n").unwrap();
    std::fs::write(
        &manual,
        format!(
            "| Benchmark sample set | Three samples | CSV saved outside repo at {} |\n",
            csv.display()
        ),
    )
    .unwrap();

    run(vec![manual.display().to_string()]).unwrap();
}

#[test]
fn packaged_rerun_keeps_observation_reminder_copy() {
    let reminder = crate::manual_qa_observation::packaged_visibility_reminder(
        "paid beta packaged observation reminder",
    );

    assert!(reminder.contains("mounted app pid count"));
    assert!(reminder.contains("license field"));
    assert!(reminder.contains("Choose recording action"));
}

#[test]
fn rejects_extra_arguments() {
    let error = run(vec!["one.md".into(), "two.md".into()]).unwrap_err();
    assert!(error.contains("manual-qa-paid-beta-rerun"));
}

#[test]
fn tolerates_missing_benchmark_csv() {
    let dir = tempfile::tempdir().unwrap();
    let manual = dir.path().join("manual.md");
    std::fs::write(
        &manual,
        "| Benchmark sample set | Three samples | no csv |\n",
    )
    .unwrap();
    run(vec![manual.display().to_string()]).unwrap();
}

#[test]
fn prints_operator_checklists_and_public_web_handoff() {
    let lines = next_lines(std::path::Path::new("/tmp/manual.md"));

    assert!(lines.iter().any(|line| line.contains("docs/manual-qa.md")));
    assert!(
        lines
            .iter()
            .any(|line| line
                .contains("next license sandbox runbook: docs/license-sandbox-runbook.md"))
    );
    assert!(lines
        .iter()
        .any(|line| line.contains("next signed DMG runbook: docs/signed-dmg-runbook.md")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Sign in to Lemon Squeezy")));
    assert!(lines
        .iter()
        .any(|line| line.contains("auth.lemonsqueezy.com/login")));
    assert!(lines.iter().any(|line| line.contains("dashboard is open")));
    assert!(lines
        .iter()
        .any(|line| line.contains("sandbox mode is visible")));
    assert!(lines
        .iter()
        .any(|line| line.contains("sandbox quickstart 2 before the UI action")));
    assert!(lines
        .iter()
        .any(|line| line.contains("sandbox quickstart 4 after Pro appears")));
    assert!(lines
        .iter()
        .any(|line| line.contains("distribution isolated target hint")));
    assert!(lines
        .iter()
        .any(|line| line.contains("CARGO_TARGET_DIR=/tmp/dsq-xtask-target")));
    assert!(lines
        .iter()
        .any(|line| line.contains("paid beta license markdown rows")));
    assert!(lines.iter().any(|line| line.contains("sandbox purchase")));
    assert!(lines
        .iter()
        .any(|line| line.contains("paid beta distribution markdown rows")));
    assert!(lines
        .iter()
        .any(|line| line.contains("distribution .*markdown row:")));
    assert!(lines
        .iter()
        .any(|line| line.contains("docs/paid-beta-operator-checklist.md")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Short Execution Memo")));
    assert!(lines
        .iter()
        .any(|line| line.contains("cargo run -p xtask -- public-web-ready")));
    assert!(lines
        .iter()
        .any(|line| line.contains("cargo run -p xtask -- public-web-rerun")));
    assert!(lines
        .iter()
        .any(|line| line.contains("productization-status --track 'Public web proof'")));
    assert!(lines
        .iter()
        .any(|line| line.contains("docs/public-beta-operator-checklist.md")));
}

#[test]
fn source_contains_deterministic_and_recovery_copy() {
    let source = std::fs::read_to_string(format!(
        "{}/src/manual_qa_paid_beta_rerun.rs",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap();

    assert!(source.contains("manual-qa-ready-all"));
    assert!(source.contains("standard paid-beta pass"));
    assert!(source.contains("manual-qa-packaged-rerun"));
    assert!(source.contains("packaged-app slice"));
    assert!(source.contains("benchmark-csv-check"));
    assert!(source.contains("manual-qa-prepare --reset-trial"));
    assert!(source.contains("next license sandbox runbook"));
    assert!(source.contains("next signed DMG runbook"));
    assert!(source.contains("Sign in to Lemon Squeezy"));
    assert!(source.contains("auth.lemonsqueezy.com/login"));
    assert!(source.contains("dashboard is open"));
    assert!(source.contains("sandbox mode is visible"));
    assert!(source.contains("sandbox quickstart 2 before the UI action"));
    assert!(source.contains("sandbox quickstart 4 after Pro appears"));
    assert!(source.contains("distribution isolated target hint"));
    assert!(source.contains("CARGO_TARGET_DIR=/tmp/dsq-xtask-target"));
    assert!(source.contains("manual_qa_license_rerun::status::lines(&path)?"));
    assert!(source.contains("manual_qa_distribution_rerun::status::lines()"));
}
