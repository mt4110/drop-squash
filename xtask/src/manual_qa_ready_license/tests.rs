use super::state::fresh_launch_command;
use super::steps::{
    BROWSER_SIGN_IN_HINT, BROWSER_SIGN_IN_SUCCESS, DISTRIBUTION_HANDOFF, MANUAL_LICENSE_ISSUANCE,
    PACKAGED_GUIDE, PAID_BETA_CHECKLIST, PUBLIC_WEB_HANDOFF, READY_ALL_HINT, SIGNED_DMG_RUNBOOK,
};
use super::{
    finish, is_tmp_path, isolation_recovery_line, license_cache_path, license_pending_command,
    manual_check_command, needs_cleaning, parse_args, sandbox_rows_command, uses_dmg_artifact,
    USAGE,
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
        license_pending_command(&path),
        "cargo run -p xtask -- manual-qa-pending '/tmp/QA Path'\\''s/ready.md' --section license"
    );
    assert_eq!(
        manual_check_command(&path),
        "cargo run -p xtask -- manual-qa-check '/tmp/QA Path'\\''s/ready.md'"
    );
    assert_eq!(
        sandbox_rows_command(&path),
        "cargo run -p xtask -- manual-qa-pending '/tmp/QA Path'\\''s/ready.md' --section license | rg '^license (product setup|sandbox purchase|valid activation) markdown row:'"
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
    std::fs::write(
        &path,
        "Prepared manual QA draft only.\n\n| App build | x |\n",
    )
    .unwrap();

    finish(&path).unwrap();

    assert_eq!(
        std::fs::read_to_string(&path).unwrap(),
        "| App build | x |\n"
    );
}

#[test]
fn reads_license_cache_path_from_markdown() {
    let text = "| License cache path | /tmp/state/license.json |\n";

    assert_eq!(
        license_cache_path(text),
        Some("/tmp/state/license.json".to_string())
    );
}

#[test]
fn builds_fresh_launch_commands_from_markdown() {
    let text = "| Config path | /tmp/state/Library/Application Support/DropSquash/config.json |\n";

    assert_eq!(
        fresh_launch_command(text, None),
        Some("cargo run -p xtask -- manual-qa-launch-app '/tmp/dsq-build-target/release/bundle/macos/DropSquash.app' '/tmp/state/Library/Application Support/DropSquash/config.json'".to_string())
    );
    assert_eq!(
        fresh_launch_command(text, Some("http://127.0.0.1:9/v1/licenses")),
        Some("cargo run -p xtask -- manual-qa-launch-app --license-api-base-url 'http://127.0.0.1:9/v1/licenses' '/tmp/dsq-build-target/release/bundle/macos/DropSquash.app' '/tmp/state/Library/Application Support/DropSquash/config.json'".to_string())
    );
}

#[test]
fn recognizes_tmp_isolation_paths() {
    assert!(is_tmp_path("/tmp/state/license.json"));
    assert!(!is_tmp_path(
        "/Users/masakitakemura/Library/Application Support/DropSquash/license.json"
    ));
}

#[test]
fn skips_recovery_command_for_tmp_paths() {
    assert_eq!(isolation_recovery_line("/tmp/state/license.json"), None);
}

#[test]
fn prints_recovery_command_for_non_tmp_paths() {
    assert_eq!(
        isolation_recovery_line(
            "/Users/masakitakemura/Library/Application Support/DropSquash/license.json"
        ),
        Some(
            "license isolation recovery command: cargo run -p xtask -- manual-qa-prepare --reset-trial --app-artifact target/release/bundle/dmg/DropSquash.dmg --input-sample-set \"short, medium, and large local recordings\" --app-state-dir \"/tmp/dropsquash-manual-qa-app-state/Library/Application Support/DropSquash\" --state-dir /tmp/dropsquash-manual-qa-state --output-dir /tmp/dropsquash-manual-qa-output --markdown-output /tmp/dropsquash-manual-qa-prepared.md"
                .to_string()
        )
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
fn markdown_row_filter_stays_stable() {
    let path = std::path::PathBuf::from("/tmp/manual.md");
    assert_eq!(
        format!(
            "{} | rg 'license .*markdown row:'",
            license_pending_command(&path)
        ),
        "cargo run -p xtask -- manual-qa-pending '/tmp/manual.md' --section license | rg 'license .*markdown row:'"
    );
}

#[test]
fn detects_dmg_artifact_for_license_note() {
    assert!(uses_dmg_artifact(
        "| App artifact | /tmp/DropSquash.dmg |\n"
    ));
    assert!(!uses_dmg_artifact(
        "| App artifact | /tmp/DropSquash.app |\n"
    ));
}

#[test]
fn license_handoff_stays_stable() {
    assert_eq!(
        MANUAL_LICENSE_ISSUANCE,
        "docs/manual-beta-license-issuance.md"
    );
    assert_eq!(PAID_BETA_CHECKLIST, "docs/paid-beta-operator-checklist.md");
    assert_eq!(PACKAGED_GUIDE, "docs/manual-qa.md");
    assert_eq!(SIGNED_DMG_RUNBOOK, "docs/signed-dmg-runbook.md");
    assert!(DISTRIBUTION_HANDOFF.contains("scripts/manual-qa-distribution-handoff.sh"));
    assert!(BROWSER_SIGN_IN_HINT.contains("Sign in to Lemon Squeezy"));
    assert!(BROWSER_SIGN_IN_HINT.contains("auth.lemonsqueezy.com/login"));
    assert!(BROWSER_SIGN_IN_SUCCESS.contains("dashboard is open"));
    assert!(BROWSER_SIGN_IN_SUCCESS.contains("sandbox mode is visible"));
    assert!(READY_ALL_HINT.contains("manual-qa-ready-all"));
    assert!(PUBLIC_WEB_HANDOFF.contains("cargo run -p xtask -- public-web-ready"));
    assert!(PUBLIC_WEB_HANDOFF.contains("cargo run -p xtask -- public-web-rerun"));
    assert!(PUBLIC_WEB_HANDOFF.contains("productization-status --track 'Public web proof'"));
    assert!(PUBLIC_WEB_HANDOFF.contains("docs/public-beta-operator-checklist.md"));
    assert!(PUBLIC_WEB_HANDOFF.contains("Short Execution Memo"));
}
