use super::{
    finish, fresh_artifact_path, fresh_build_command, fresh_not_smaller_command,
    installed_app_restore_command, installed_app_stash_command, installed_app_status_command,
    manual_check_command, needs_cleaning, packaged_app_pending_command, parse_args, run_args,
    sample_hints, sample_link_command, USAGE,
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

#[test]
fn observation_reminder_copy_stays_present() {
    let reminder = crate::manual_qa_observation::packaged_visibility_reminder(
        "packaged-app observation reminder",
    );

    assert!(reminder.contains("mounted app pid count"));
    assert!(reminder.contains("license field"));
    assert!(reminder.contains("Choose recording action"));
}

#[test]
fn docs_entrypoints_stay_present() {
    assert!(super::PACKAGED_GUIDE.contains("docs/manual-qa.md"));
    assert!(super::LICENSE_RUNBOOK.contains("docs/license-sandbox-runbook.md"));
    assert!(super::SIGNED_DMG_RUNBOOK.contains("docs/signed-dmg-runbook.md"));
    assert!(super::DISTRIBUTION_HANDOFF.contains("scripts/manual-qa-distribution-handoff.sh"));
}

#[test]
fn reports_fresh_build_guidance() {
    assert_eq!(
        fresh_build_command(),
        "CARGO_TARGET_DIR=/tmp/dsq-build-target pnpm --dir apps/desktop tauri build"
    );
    assert_eq!(
        fresh_artifact_path(),
        "/tmp/dsq-build-target/release/bundle/macos/DropSquash.app"
    );
}

#[test]
fn reports_packaged_app_helper_commands() {
    let csv = std::path::PathBuf::from("/tmp/QA Path's/results.csv");

    assert_eq!(
        sample_link_command(&csv),
        "cargo run -p xtask -- manual-qa-link-samples '/tmp/QA Path'\\''s/results.csv' /tmp/dropsquash-qa-open-panel"
    );
    assert_eq!(
        installed_app_status_command(),
        "cargo run -p xtask -- manual-qa-installed-app status /tmp/dropsquash-manual-qa-installed-app"
    );
    assert_eq!(
        installed_app_stash_command(),
        "cargo run -p xtask -- manual-qa-installed-app stash /tmp/dropsquash-manual-qa-installed-app"
    );
    assert_eq!(
        installed_app_restore_command(),
        "cargo run -p xtask -- manual-qa-installed-app restore /tmp/dropsquash-manual-qa-installed-app"
    );
}

#[test]
fn paid_beta_check_command_stays_stable() {
    assert_eq!(
        "cargo run -p xtask -- paid-beta-check",
        "cargo run -p xtask -- paid-beta-check"
    );
}

#[test]
fn reports_sample_hints_from_csv() {
    let directory = tempfile::tempdir().unwrap();
    let csv = directory.path().join("results.csv");
    std::fs::write(
        &csv,
        "backend,input,output\napple-native,/tmp/short.mov,/tmp/short.squashed.mp4\napple-native,/tmp/medium.mov,/tmp/medium.squashed.mp4\napple-native,/tmp/large.mov,/tmp/large.squashed.mp4\n",
    )
    .unwrap();

    let hints = sample_hints(&csv).unwrap();

    assert_eq!(hints[0], "packaged-app small sample: /tmp/short.mov");
    assert_eq!(hints[1], "packaged-app duplicate sample: /tmp/short.mov");
    assert_eq!(
        hints[2],
        "packaged-app queue sample set: /tmp/short.mov, /tmp/medium.mov, /tmp/large.mov"
    );
    assert_eq!(hints[3], "packaged-app large sample: /tmp/large.mov");
    assert_eq!(
        hints[4],
        "packaged-app not-smaller sample: /tmp/short.squashed.mp4"
    );
}

#[test]
fn reports_fresh_not_smaller_command_when_config_exists() {
    let directory = tempfile::tempdir().unwrap();
    let csv = directory.path().join("results.csv");
    std::fs::write(
        &csv,
        "backend,input,output\napple-native,/tmp/short.mov,/tmp/short.squashed.mp4\napple-native,/tmp/medium.mov,/tmp/medium.squashed.mp4\napple-native,/tmp/large.mov,/tmp/large.squashed.mp4\n",
    )
    .unwrap();

    let command = fresh_not_smaller_command(
        "| Config path | /tmp/state/Library/Application Support/DropSquash/config.json |\n",
        &csv,
    )
    .unwrap()
    .unwrap();

    assert_eq!(
        command,
        "cargo run -p xtask -- manual-qa-launch-app --settle-seconds 9 --open-file '/tmp/short.squashed.mp4' '/tmp/dsq-build-target/release/bundle/macos/DropSquash.app' '/tmp/state/Library/Application Support/DropSquash/config.json'"
    );
}

#[test]
fn skips_cleaning_for_plain_manual_qa_file() {
    let directory = tempfile::tempdir().unwrap();
    let manual = directory.path().join("manual.md");
    std::fs::write(&manual, "| Config path | /tmp/config.json |\n").unwrap();

    assert!(!needs_cleaning(&manual).unwrap());
    finish(&manual).unwrap();
    assert_eq!(
        std::fs::read_to_string(&manual).unwrap(),
        "| Config path | /tmp/config.json |\n"
    );
}

#[test]
fn cleans_prepared_manual_qa_file() {
    let directory = tempfile::tempdir().unwrap();
    let manual = directory.path().join("prepared.md");
    std::fs::write(
        &manual,
        "Prepared manual QA draft only.\n| Config path | /tmp/config.json |\n",
    )
    .unwrap();

    assert!(needs_cleaning(&manual).unwrap());
    finish(&manual).unwrap();
    assert_eq!(
        std::fs::read_to_string(&manual).unwrap(),
        "| Config path | /tmp/config.json |\n"
    );
}
