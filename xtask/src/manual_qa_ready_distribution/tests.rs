use super::{
    distribution_pending_command, finish, manual_check_command, needs_cleaning, parse_args, USAGE,
};

#[test]
fn parses_markdown_path() {
    let path = parse_args(vec!["/tmp/manual.md".into()]).unwrap();
    assert_eq!(path, std::path::PathBuf::from("/tmp/manual.md"));
}

#[test]
fn rejects_invalid_arguments() {
    assert_eq!(parse_args(vec![]).unwrap_err(), USAGE);
}

#[test]
fn quotes_followup_commands() {
    let path = std::path::PathBuf::from("/tmp/QA Path's/ready.md");

    assert_eq!(
        distribution_pending_command(&path),
        "cargo run -p xtask -- manual-qa-pending '/tmp/QA Path'\\''s/ready.md' --section distribution"
    );
    assert_eq!(
        manual_check_command(&path),
        "cargo run -p xtask -- manual-qa-check '/tmp/QA Path'\\''s/ready.md'"
    );
}

#[test]
fn detects_prepared_draft_marker() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("manual.md");
    std::fs::write(&path, "Prepared manual QA draft only.\n| row |\n").unwrap();

    assert!(needs_cleaning(&path).unwrap());
}

#[test]
fn ignores_cleaned_manual_file() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("manual.md");
    std::fs::write(&path, "| row |\n").unwrap();

    assert!(!needs_cleaning(&path).unwrap());
}

#[test]
fn finish_keeps_cleaned_manual_file() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("manual.md");
    std::fs::write(&path, "| App build | x |\n| Check | Expected | Result |\n").unwrap();

    finish(&path).unwrap();

    assert_eq!(
        std::fs::read_to_string(&path).unwrap(),
        "| App build | x |\n| Check | Expected | Result |\n"
    );
}

#[test]
fn finish_cleans_prepared_draft() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("manual.md");
    std::fs::write(&path, "Prepared manual QA draft only.\n\n| App build | x |\n").unwrap();

    finish(&path).unwrap();

    assert_eq!(std::fs::read_to_string(&path).unwrap(), "| App build | x |\n");
}
