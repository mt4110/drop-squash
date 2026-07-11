use std::path::PathBuf;

use super::{prepare, Options};

#[test]
fn creates_output_and_backs_up_existing_state_files() {
    let directory = tempfile::tempdir().unwrap();
    let app_state_dir = directory.path().join("app-state");
    let state_dir = directory.path().join("state");
    let output_dir = directory.path().join("output");
    std::fs::create_dir(&app_state_dir).unwrap();
    std::fs::write(app_state_dir.join("config.json"), "config").unwrap();
    std::fs::write(app_state_dir.join("history.jsonl"), "history").unwrap();

    let copied = prepare(&Options {
        app_state_dir,
        output_dir: output_dir.clone(),
        reset_trial: false,
        state_dir: state_dir.clone(),
    })
    .unwrap();

    assert_eq!(copied, vec!["config.json", "history.jsonl"]);
    assert_eq!(
        std::fs::read_to_string(state_dir.join("config.json")).unwrap(),
        "config"
    );
    assert!(output_dir.is_dir());
}

#[test]
fn skips_missing_state_files() {
    let directory = tempfile::tempdir().unwrap();
    let app_state_dir = directory.path().join("missing-app-state");
    let state_dir = directory.path().join("state");
    let output_dir = directory.path().join("output");

    let copied = prepare(&Options {
        app_state_dir,
        output_dir,
        reset_trial: false,
        state_dir,
    })
    .unwrap();

    assert!(copied.is_empty());
}

#[test]
fn parses_custom_directories() {
    let options = Options::parse(vec![
        "--app-state-dir".to_string(),
        "/tmp/app-state".to_string(),
        "--reset-trial".to_string(),
        "--output-dir".to_string(),
        "/tmp/output".to_string(),
        "--state-dir".to_string(),
        "/tmp/state".to_string(),
    ])
    .unwrap();

    assert_eq!(options.app_state_dir, PathBuf::from("/tmp/app-state"));
    assert_eq!(options.output_dir, PathBuf::from("/tmp/output"));
    assert!(options.reset_trial);
    assert_eq!(options.state_dir, PathBuf::from("/tmp/state"));
}

#[test]
fn resets_trial_state_after_backup_when_requested() {
    let directory = tempfile::tempdir().unwrap();
    let app_state_dir = directory.path().join("app-state");
    let state_dir = directory.path().join("state");
    let output_dir = directory.path().join("output");
    std::fs::create_dir(&app_state_dir).unwrap();
    std::fs::write(app_state_dir.join("config.json"), "config").unwrap();
    std::fs::write(app_state_dir.join("history.jsonl"), "history").unwrap();
    std::fs::write(app_state_dir.join("license.json"), "license").unwrap();

    let copied = prepare(&Options {
        app_state_dir: app_state_dir.clone(),
        output_dir,
        reset_trial: true,
        state_dir: state_dir.clone(),
    })
    .unwrap();

    assert_eq!(copied, vec!["config.json", "history.jsonl", "license.json"]);
    assert!(app_state_dir.join("config.json").exists());
    assert!(!app_state_dir.join("history.jsonl").exists());
    assert!(!app_state_dir.join("license.json").exists());
    assert_eq!(
        std::fs::read_to_string(state_dir.join("history.jsonl")).unwrap(),
        "history"
    );
}

#[test]
fn rejects_unknown_arguments() {
    let error = Options::parse(vec!["--mystery".to_string(), "value".to_string()]).unwrap_err();

    assert!(error.contains("unknown manual QA prepare argument"));
}
