use super::{manual_check_command, packaged_app_pending_command, parse_args, run_args, USAGE};

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
        run_args(&parsed),
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
fn quotes_followup_commands() {
    let path = std::path::PathBuf::from("/tmp/QA Path's/ready.md");

    assert_eq!(
        packaged_app_pending_command(&path),
        "cargo run -p xtask -- manual-qa-pending '/tmp/QA Path'\\''s/ready.md' --section packaged-app"
    );
    assert_eq!(
        manual_check_command(&path),
        "cargo run -p xtask -- manual-qa-check '/tmp/QA Path'\\''s/ready.md'"
    );
}
