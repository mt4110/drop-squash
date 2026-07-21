use std::path::PathBuf;

use super::{parse_args, run};

#[test]
fn defaults_to_main_manual_qa() {
    assert_eq!(
        parse_args(vec![]).unwrap(),
        PathBuf::from("docs/manual-qa.md")
    );
}

#[test]
fn accepts_explicit_manual_path() {
    assert_eq!(
        parse_args(vec!["/tmp/prepared.md".into()]).unwrap(),
        PathBuf::from("/tmp/prepared.md")
    );
}

#[test]
fn rejects_extra_arguments() {
    let error = run(vec!["one.md".into(), "two.md".into()]).unwrap_err();
    assert!(error.contains("manual-qa-license-rerun"));
}

#[test]
fn source_keeps_dirty_worktree_quickstart_hook() {
    let source = std::fs::read_to_string(format!(
        "{}/src/manual_qa_license_rerun.rs",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap();

    assert!(source.contains("manual_qa_dirty_worktree::print_quickstart()"));
    assert!(source.contains("docs/license-sandbox-runbook.md"));
    assert!(source.contains("status::lines(&path)?"));
}
