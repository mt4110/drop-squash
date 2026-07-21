use super::{
    benchmark_command, display, ensure_current_csv_exists, local_proof_args, next_lines,
    parse_args, ISOLATED_PREPARE, USAGE,
};

#[test]
fn parses_without_baseline() {
    let parsed = parse_args(vec!["/tmp/manual.md".into(), "/tmp/results.csv".into()]).unwrap();

    assert_eq!(parsed.0, std::path::PathBuf::from("/tmp/manual.md"));
    assert_eq!(parsed.1, std::path::PathBuf::from("/tmp/results.csv"));
    assert_eq!(parsed.2, None);
}

#[test]
fn forwards_optional_baseline() {
    let parsed = parse_args(vec![
        "/tmp/manual.md".into(),
        "/tmp/results.csv".into(),
        "/tmp/baseline.csv".into(),
    ])
    .unwrap();

    assert_eq!(
        local_proof_args(&parsed),
        vec![
            "/tmp/manual.md".to_string(),
            "/tmp/results.csv".to_string(),
            "/tmp/baseline.csv".to_string()
        ]
    );
}

#[test]
fn rejects_invalid_arguments() {
    assert_eq!(parse_args(vec![]).unwrap_err(), USAGE);
}

#[test]
fn paid_beta_check_command_stays_stable() {
    assert_eq!(
        "cargo run -p xtask -- paid-beta-check",
        "cargo run -p xtask -- paid-beta-check"
    );
}

#[test]
fn display_preserves_path_text() {
    assert_eq!(
        display(&std::path::PathBuf::from("/tmp/manual.md")),
        "/tmp/manual.md"
    );
}

#[test]
fn next_lines_include_paid_beta_follow_up_commands() {
    let lines = next_lines(&std::path::PathBuf::from("/tmp/manual qa.md"));

    assert!(lines
        .iter()
        .any(|line| line.contains("docs/paid-beta-operator-checklist.md")));
    assert!(lines.iter().any(|line| line.contains("docs/manual-qa.md")));
    assert!(lines
        .iter()
        .any(|line| line.contains("docs/license-sandbox-runbook.md")));
    assert!(lines
        .iter()
        .any(|line| line.contains("docs/signed-dmg-runbook.md")));
    assert!(lines
        .iter()
        .any(|line| line.contains("scripts/manual-qa-distribution-handoff.sh")));
    assert!(lines
        .iter()
        .any(|line| { line.contains("cargo run -p xtask -- manual-qa-paid-beta-rerun") }));
    assert!(lines
        .iter()
        .any(|line| line.contains("manual-qa-pending '/tmp/manual qa.md' --section license")));
    assert!(lines.iter().any(|line| {
        line.contains("manual-qa-pending '/tmp/manual qa.md' --section distribution")
    }));
    assert!(lines
        .iter()
        .any(|line| line.contains("manual-qa-check '/tmp/manual qa.md'")));
    assert!(lines
        .iter()
        .any(|line| line.contains("cargo run -p xtask -- paid-beta-check")));
}

#[test]
fn reports_missing_csv_with_recovery_command() {
    let error = ensure_current_csv_exists(&std::path::PathBuf::from("/tmp/missing-results.csv"))
        .unwrap_err();

    assert!(error.contains("benchmark CSV does not exist"));
    assert!(error.contains("/tmp/missing-results.csv"));
    assert!(error.contains(ISOLATED_PREPARE));
    assert!(error.contains("benchmark --release-set"));
    assert!(error.contains("benchmark-csv-check /tmp/missing-results.csv"));
    assert!(error.contains("manual-qa-ready-all"));
}

#[test]
fn benchmark_command_reuses_csv_parent_for_output_dir() {
    let command = benchmark_command(&std::path::PathBuf::from(
        "/tmp/dsq-output/benchmark-results-abc1234.csv",
    ));

    assert!(command.contains("--output-dir /tmp/dsq-output"));
    assert!(command.contains("--csv-output /tmp/dsq-output/benchmark-results-abc1234.csv"));
}
