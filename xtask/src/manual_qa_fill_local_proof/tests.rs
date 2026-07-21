use super::{fill_local_proof_row, parse_args, USAGE};

#[test]
fn parses_current_csv_without_baseline() {
    let (manual, current, baseline) =
        parse_args(vec!["/tmp/manual.md".into(), "/tmp/results.csv".into()]).unwrap();

    assert_eq!(manual, std::path::PathBuf::from("/tmp/manual.md"));
    assert_eq!(current, std::path::PathBuf::from("/tmp/results.csv"));
    assert_eq!(baseline, None);
}

#[test]
fn parses_optional_baseline_csv() {
    let (_, _, baseline) = parse_args(vec![
        "/tmp/manual.md".into(),
        "/tmp/results.csv".into(),
        "/tmp/baseline.csv".into(),
    ])
    .unwrap();

    assert_eq!(
        baseline,
        Some(std::path::PathBuf::from("/tmp/baseline.csv"))
    );
}

#[test]
fn rejects_invalid_arguments() {
    assert_eq!(parse_args(vec![]).unwrap_err(), USAGE);
}

#[test]
fn fills_local_proof_gate_row() {
    let filled = fill_local_proof_row(
        "| `cargo run -p xtask -- manual-qa-check <manual-qa.md> --section local-proof` | Passes after local packaged-app proof is recorded |  |\n",
    )
    .unwrap();

    assert!(filled.contains("manual-qa-check --section local-proof passed"));
}
