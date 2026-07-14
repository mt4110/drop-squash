use std::path::PathBuf;

use super::options::Options;
use super::output::{manual_check_line, open_artifact_line, sample_set_line};
use super::output_helper::{
    clean_draft_line, fill_benchmark_line, fill_benchmark_threshold_line, fill_check_line,
    fill_release_gates_line, pending_benchmark_line, pending_distribution_line, pending_line,
    pending_license_line, pending_packaged_app_line,
};
use super::require_reset_artifact;
use super::reset_trial_lines;
use super::state::{backup_state, restore_state};
use super::{markdown, release_candidate};

#[test]
fn creates_output_and_backs_up_existing_state_files() {
    let directory = tempfile::tempdir().unwrap();
    let app_state_dir = directory.path().join("app-state");
    let state_dir = directory.path().join("state");
    let output_dir = directory.path().join("output");
    std::fs::create_dir(&app_state_dir).unwrap();
    std::fs::write(app_state_dir.join("config.json"), "config").unwrap();
    std::fs::write(app_state_dir.join("history.jsonl"), "history").unwrap();

    let copied = backup_state(&Options {
        app_artifact: None,
        app_state_dir,
        input_sample_set: None,
        markdown_output: None,
        output_dir: output_dir.clone(),
        reset_trial: false,
        restore_state: false,
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

    let copied = backup_state(&Options {
        app_artifact: None,
        app_state_dir,
        input_sample_set: None,
        markdown_output: None,
        output_dir,
        reset_trial: false,
        restore_state: false,
        state_dir,
    })
    .unwrap();

    assert!(copied.is_empty());
}

#[test]
fn parses_custom_directories() {
    let options = Options::parse(vec![
        "--app-state-dir".to_string(),
        "/tmp/Library/Application Support/DropSquash".to_string(),
        "--app-artifact".to_string(),
        "/tmp/DropSquash.app".to_string(),
        "--input-sample-set".to_string(),
        "short, medium, and large local recordings".to_string(),
        "--reset-trial".to_string(),
        "--markdown-output".to_string(),
        "/tmp/manual-qa-prepared.md".to_string(),
        "--output-dir".to_string(),
        "/tmp/output".to_string(),
        "--state-dir".to_string(),
        "/tmp/state".to_string(),
    ])
    .unwrap();

    assert_eq!(
        options.app_artifact,
        Some(PathBuf::from("/tmp/DropSquash.app"))
    );
    assert_eq!(
        options.input_sample_set,
        Some("short, medium, and large local recordings".to_string())
    );
    assert_eq!(
        options.markdown_output,
        Some(PathBuf::from("/tmp/manual-qa-prepared.md"))
    );
    assert_eq!(
        options.app_state_dir,
        PathBuf::from("/tmp/Library/Application Support/DropSquash")
    );
    assert_eq!(options.output_dir, PathBuf::from("/tmp/output"));
    assert!(options.reset_trial);
    assert!(!options.restore_state);
    assert_eq!(options.state_dir, PathBuf::from("/tmp/state"));
}

#[test]
fn prints_sample_set_prompt_when_missing() {
    let options = Options::parse(Vec::new()).unwrap();

    assert!(sample_set_line(&options).contains("--input-sample-set"));
}

#[test]
fn prints_provided_sample_set() {
    let options = Options::parse(vec![
        "--input-sample-set".to_string(),
        "short, medium, and large local recordings".to_string(),
    ])
    .unwrap();

    assert_eq!(
        sample_set_line(&options),
        "manual QA Input sample set: short, medium, and large local recordings"
    );
}

#[test]
fn manual_check_output_quotes_markdown_path() {
    let line = manual_check_line(&PathBuf::from("/tmp/QA Path's/prepared.md"));

    assert_eq!(
        line,
        "manual QA Check command: cargo run -p xtask -- manual-qa-check '/tmp/QA Path'\\''s/prepared.md'"
    );
}

#[test]
fn helper_output_quotes_paths() {
    let markdown = PathBuf::from("/tmp/QA Path's/prepared.md");
    let output_dir = PathBuf::from("/tmp/Output Path's");

    assert_eq!(
        fill_release_gates_line(&markdown),
        "manual QA Fill release gates command: cargo run -p xtask -- manual-qa-fill-release-gates '/tmp/QA Path'\\''s/prepared.md'"
    );
    assert_eq!(
        fill_benchmark_line(&markdown, &output_dir),
        "manual QA Fill benchmark command: cargo run -p xtask -- manual-qa-fill-benchmark '/tmp/QA Path'\\''s/prepared.md' '/tmp/Output Path'\\''s/benchmark-results.csv'"
    );
    assert_eq!(
        fill_benchmark_threshold_line(&markdown, &output_dir),
        "manual QA Fill benchmark threshold command: cargo run -p xtask -- manual-qa-fill-benchmark-threshold '/tmp/QA Path'\\''s/prepared.md' '/tmp/Output Path'\\''s/benchmark-results.csv'"
    );
    assert_eq!(
        clean_draft_line(&markdown),
        "manual QA Clean draft command: cargo run -p xtask -- manual-qa-clean-draft '/tmp/QA Path'\\''s/prepared.md'"
    );
    assert_eq!(
        pending_line(&markdown),
        "manual QA Pending command: cargo run -p xtask -- manual-qa-pending '/tmp/QA Path'\\''s/prepared.md'"
    );
    assert_eq!(
        pending_packaged_app_line(&markdown),
        "manual QA Packaged App pending command: cargo run -p xtask -- manual-qa-pending '/tmp/QA Path'\\''s/prepared.md' --section packaged-app"
    );
    assert_eq!(
        pending_license_line(&markdown),
        "manual QA License pending command: cargo run -p xtask -- manual-qa-pending '/tmp/QA Path'\\''s/prepared.md' --section license"
    );
    assert_eq!(
        pending_benchmark_line(&markdown),
        "manual QA Benchmark pending command: cargo run -p xtask -- manual-qa-pending '/tmp/QA Path'\\''s/prepared.md' --section benchmark"
    );
    assert_eq!(
        pending_distribution_line(&markdown),
        "manual QA Distribution pending command: cargo run -p xtask -- manual-qa-pending '/tmp/QA Path'\\''s/prepared.md' --section distribution"
    );
    assert_eq!(
        fill_check_line(&markdown),
        "manual QA Fill check command: cargo run -p xtask -- manual-qa-fill-check '/tmp/QA Path'\\''s/prepared.md'"
    );
}

#[test]
fn open_artifact_output_quotes_dmg_path() {
    let line = open_artifact_line(&PathBuf::from("/tmp/QA Path's/DropSquash.dmg")).unwrap();

    assert_eq!(
        line,
        "manual QA Open artifact command: open -- '/tmp/QA Path'\\''s/DropSquash.dmg'"
    );
}

#[test]
fn open_artifact_output_ignores_non_dmg_artifact() {
    assert!(open_artifact_line(&PathBuf::from("/tmp/DropSquash.app")).is_none());
}

#[test]
fn reset_trial_output_names_state_directory_and_files() {
    let options = Options::parse(vec![
        "--app-state-dir".to_string(),
        "/tmp/Library/Application Support/DropSquash".to_string(),
        "--input-sample-set".to_string(),
        "short, medium, and large local recordings".to_string(),
        "--reset-trial".to_string(),
    ])
    .unwrap();

    let lines = reset_trial_lines(&options).join("\n");

    assert!(lines.contains("/tmp/Library/Application Support/DropSquash"));
    assert!(lines.contains("history.jsonl"));
    assert!(lines.contains("license.json"));
    assert!(lines.contains(
        "manual-qa-prepare --restore-state --app-state-dir '/tmp/Library/Application Support/DropSquash'"
    ));
    assert!(lines.contains("--state-dir '/tmp/dropsquash-qa-state'"));
}

#[test]
fn parses_restore_state() {
    let options = Options::parse(vec!["--restore-state".to_string()]).unwrap();

    assert!(!options.reset_trial);
    assert!(options.restore_state);
}

#[test]
fn restore_rejects_prepare_only_inputs() {
    let error = Options::parse(vec![
        "--restore-state".to_string(),
        "--input-sample-set".to_string(),
        "short, medium, and large local recordings".to_string(),
    ])
    .unwrap_err();

    assert!(error.contains("--restore-state only accepts"));
}

#[test]
fn prints_usage_for_help() {
    let error = Options::parse(vec!["--help".to_string()]).unwrap_err();

    assert!(error.contains("usage: cargo run -p xtask -- manual-qa-prepare"));
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

    let copied = backup_state(&Options {
        app_artifact: None,
        app_state_dir: app_state_dir.clone(),
        input_sample_set: None,
        markdown_output: None,
        output_dir,
        reset_trial: true,
        restore_state: false,
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
fn restores_backed_up_state_files() {
    let directory = tempfile::tempdir().unwrap();
    let app_state_dir = directory.path().join("app-state");
    let state_dir = directory.path().join("state");
    let output_dir = directory.path().join("output");
    std::fs::create_dir(&state_dir).unwrap();
    std::fs::write(state_dir.join("config.json"), "config").unwrap();
    std::fs::write(state_dir.join("license.json"), "license").unwrap();

    let copied = restore_state(&Options {
        app_artifact: None,
        app_state_dir: app_state_dir.clone(),
        input_sample_set: None,
        markdown_output: None,
        output_dir,
        reset_trial: false,
        restore_state: true,
        state_dir,
    })
    .unwrap();

    assert_eq!(copied, vec!["config.json", "license.json"]);
    assert_eq!(
        std::fs::read_to_string(app_state_dir.join("license.json")).unwrap(),
        "license"
    );
}

#[test]
fn restore_requires_existing_backup_directory() {
    let directory = tempfile::tempdir().unwrap();
    let error = restore_state(&Options {
        app_artifact: None,
        app_state_dir: directory.path().join("app-state"),
        input_sample_set: None,
        markdown_output: None,
        output_dir: directory.path().join("output"),
        reset_trial: false,
        restore_state: true,
        state_dir: directory.path().join("missing-state"),
    })
    .unwrap_err();

    assert!(error.contains("state backup does not exist"));
}

#[test]
fn restore_requires_restorable_state_files() {
    let directory = tempfile::tempdir().unwrap();
    let state_dir = directory.path().join("state");
    std::fs::create_dir(&state_dir).unwrap();

    let error = restore_state(&Options {
        app_artifact: None,
        app_state_dir: directory.path().join("app-state"),
        input_sample_set: None,
        markdown_output: None,
        output_dir: directory.path().join("output"),
        reset_trial: false,
        restore_state: true,
        state_dir,
    })
    .unwrap_err();

    assert!(error.contains("no restorable files"));
}

#[test]
fn rejects_reset_and_restore_together() {
    let error = Options::parse(vec![
        "--reset-trial".to_string(),
        "--restore-state".to_string(),
    ])
    .unwrap_err();

    assert!(error.contains("cannot be combined"));
}

#[test]
fn reset_trial_requires_sample_set() {
    let error = Options::parse(vec!["--reset-trial".to_string()]).unwrap_err();

    assert!(error.contains("--input-sample-set"));
}

#[test]
fn reset_trial_requires_existing_artifact() {
    let directory = tempfile::tempdir().unwrap();
    let options = Options::parse(vec![
        "--reset-trial".to_string(),
        "--input-sample-set".to_string(),
        "short, medium, and large local recordings".to_string(),
        "--app-artifact".to_string(),
        directory.path().join("missing.app").display().to_string(),
    ])
    .unwrap();

    let error = require_reset_artifact(&options).unwrap_err();

    assert!(error.contains("DropSquash.app"));
}

#[test]
fn reset_trial_accepts_existing_dmg_artifact() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.dmg");
    std::fs::write(&artifact, dmg_bytes(b"dropsquash")).unwrap();
    let options = Options::parse(vec![
        "--reset-trial".to_string(),
        "--input-sample-set".to_string(),
        "short, medium, and large local recordings".to_string(),
        "--app-artifact".to_string(),
        artifact.display().to_string(),
    ])
    .unwrap();

    require_reset_artifact(&options).unwrap();
}

#[test]
fn generated_manual_qa_rows_work_together() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.dmg");
    let output = directory.path().join("output");
    std::fs::write(&artifact, dmg_bytes(b"dropsquash")).unwrap();
    std::fs::create_dir(&output).unwrap();
    let mut rows = markdown::rows(&manual_qa_fields(&artifact, &output));
    rows.extend(release_candidate::rows(&artifact).unwrap());
    let path = directory.path().join("manual-qa.md");
    std::fs::write(&path, rows.join("\n")).unwrap();

    let missing = crate::manual_qa_check::check_file(&path).unwrap();

    assert!(!missing.iter().any(|error| error.contains("manual QA App")));
    assert!(!missing
        .iter()
        .any(|error| error.contains("manual QA field")));
    assert!(!missing.iter().any(|error| error.contains("artifact-check")));
    assert!(!missing.iter().any(|error| error.contains("checksum")));
}

#[test]
fn rejects_unknown_arguments() {
    let error = Options::parse(vec!["--mystery".to_string(), "value".to_string()]).unwrap_err();

    assert!(error.contains("unknown manual QA prepare argument"));
}

#[test]
fn rejects_weak_input_sample_set() {
    let error = Options::parse(vec![
        "--input-sample-set".to_string(),
        "local files".to_string(),
    ])
    .unwrap_err();

    assert!(error.contains("short, medium, and large"));
}

#[test]
fn rejects_non_local_input_sample_set() {
    let error = Options::parse(vec![
        "--input-sample-set".to_string(),
        "short, medium, and large recordings".to_string(),
    ])
    .unwrap_err();

    assert!(error.contains("local"));
}

#[test]
fn rejects_state_backup_inside_repository() {
    let error = Options::parse(vec![
        "--state-dir".to_string(),
        std::env::current_dir()
            .unwrap()
            .join("tmp/manual-qa-state")
            .display()
            .to_string(),
    ])
    .unwrap_err();

    assert!(error.contains("--state-dir"));
    assert!(error.contains("outside the repository"));
}

#[test]
fn rejects_app_state_source_inside_repository() {
    let error = Options::parse(vec![
        "--app-state-dir".to_string(),
        std::env::current_dir()
            .unwrap()
            .join("tmp/app-state")
            .display()
            .to_string(),
    ])
    .unwrap_err();

    assert!(error.contains("--app-state-dir"));
    assert!(error.contains("outside the repository"));
}

#[test]
fn rejects_app_state_source_outside_application_support() {
    let error = Options::parse(vec![
        "--app-state-dir".to_string(),
        "/tmp/dropsquash-state".to_string(),
    ])
    .unwrap_err();

    assert!(error.contains("--app-state-dir"));
    assert!(error.contains("Application Support/DropSquash"));
}

#[test]
fn rejects_output_folder_inside_repository() {
    let error = Options::parse(vec![
        "--output-dir".to_string(),
        std::env::current_dir()
            .unwrap()
            .join("tmp/manual-qa-output")
            .display()
            .to_string(),
    ])
    .unwrap_err();

    assert!(error.contains("--output-dir"));
    assert!(error.contains("outside the repository"));
}

#[test]
fn rejects_markdown_output_inside_repository() {
    let error = Options::parse(vec![
        "--markdown-output".to_string(),
        std::env::current_dir()
            .unwrap()
            .join("tmp/manual-qa-prepared.md")
            .display()
            .to_string(),
    ])
    .unwrap_err();

    assert!(error.contains("--markdown-output"));
    assert!(error.contains("outside the repository"));
}

#[test]
fn rejects_relative_state_backup_path() {
    let error = Options::parse(vec![
        "--state-dir".to_string(),
        "../manual-qa-state".to_string(),
    ])
    .unwrap_err();

    assert!(error.contains("--state-dir"));
    assert!(error.contains("absolute path"));
}

#[test]
fn rejects_markdown_output_without_markdown_extension() {
    let error = Options::parse(vec![
        "--markdown-output".to_string(),
        "/tmp/manual-qa-prepared.txt".to_string(),
    ])
    .unwrap_err();

    assert!(error.contains("--markdown-output"));
    assert!(error.contains(".md"));
}

fn dmg_bytes(prefix: &[u8]) -> Vec<u8> {
    let mut bytes = prefix.to_vec();
    let mut trailer = vec![0; 512];
    trailer[..4].copy_from_slice(b"koly");
    bytes.extend(trailer);
    bytes
}

fn manual_qa_fields(artifact: &std::path::Path, output: &std::path::Path) -> Vec<markdown::Field> {
    vec![
        (
            "App build",
            format!("DropSquash 0.1.0 git {}", current_head()),
        ),
        ("App artifact", artifact.display().to_string()),
        ("macOS version", "macOS 26.5.2".into()),
        ("Machine", "MacBookPro18,4 arm64".into()),
        (
            "Input sample set",
            "short, medium, and large local recordings".into(),
        ),
        ("Output folder", output.display().to_string()),
        ("Config path", state_path("config.json")),
        ("History path", state_path("history.jsonl")),
        ("License cache path", state_path("license.json")),
        ("Tester", "masaki".into()),
        ("Date", "2026-07-12".into()),
    ]
}

fn state_path(file_name: &str) -> String {
    format!("/Users/me/Library/Application Support/DropSquash/{file_name}")
}

fn current_head() -> String {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--short=7", "HEAD"])
        .output()
        .unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}
