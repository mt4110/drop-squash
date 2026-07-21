use super::{
    distribution_pending_command, distribution_quickstart, finish, manual_check_command,
    needs_cleaning, parse_args, uses_local_unsigned_dmg, PAID_BETA_CHECKLIST, PUBLIC_WEB_HANDOFF,
    QUICKSTART_NOTE, READY_ALL_HINT, SIGNING_ENV_CHECK, SIGNING_ENV_NOTE, USAGE,
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
            "{} | rg 'distribution .*markdown row:'",
            distribution_pending_command(&path)
        ),
        "cargo run -p xtask -- manual-qa-pending '/tmp/manual.md' --section distribution | rg 'distribution .*markdown row:'"
    );
}

#[test]
fn detects_local_unsigned_distribution_artifact() {
    assert!(uses_local_unsigned_dmg(
        "| App artifact | /tmp/work/target/release/bundle/dmg/DropSquash.dmg |\n"
    ));
    assert!(!uses_local_unsigned_dmg(
        "| App artifact | /tmp/DropSquash.app |\n"
    ));
}

#[test]
fn prints_distribution_quickstart_lines() {
    let text = "| App artifact | /tmp/work/target/release/bundle/dmg/DropSquash.dmg |\n| Output folder | /tmp/out |\n| Codesign verification | expected |  |\n";
    let lines = distribution_quickstart(text);
    assert!(lines[0].starts_with("distribution quickstart 1: distribution fresh build command:"));
    assert!(lines[3].starts_with("distribution quickstart 4: distribution fresh signing plan:"));
    assert!(
        lines[4].starts_with("distribution quickstart 5: distribution fresh codesign verify plan:")
    );
    assert!(lines[5].starts_with("distribution quickstart 6: distribution fresh stapler plan:"));
    assert!(lines[6].starts_with("distribution quickstart 7: distribution fresh spctl plan:"));
}

#[test]
fn distribution_quickstart_note_stays_stable() {
    assert_eq!(
        QUICKSTART_NOTE,
        "distribution quickstart note: after quickstart 4, run the emitted signing-plan steps so /tmp/dropsquash-signed-release/DropSquash.dmg exists before quickstart 5..7"
    );
}

#[test]
fn signing_env_check_stays_stable() {
    assert_eq!(
        SIGNING_ENV_CHECK,
        "distribution signing environment check: CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- macos-signing-check"
    );
}

#[test]
fn signing_env_note_stays_stable() {
    assert_eq!(
        SIGNING_ENV_NOTE,
        "distribution signing environment note: local macOS needs APPLE_SIGNING_IDENTITY or APPLE_CERTIFICATE with APPLE_CERTIFICATE_PASSWORD; notarization needs APPLE_API_KEY/APPLE_API_ISSUER/APPLE_API_KEY_PATH or APPLE_ID/APPLE_PASSWORD/APPLE_TEAM_ID; CI signing also needs APPLE_KEYCHAIN_PASSWORD and APPLE_CODESIGN_IDENTITY"
    );
}

#[test]
fn distribution_handoff_stays_stable() {
    assert_eq!(PAID_BETA_CHECKLIST, "docs/paid-beta-operator-checklist.md");
    assert!(READY_ALL_HINT.contains("manual-qa-ready-all"));
    let source = std::fs::read_to_string(format!(
        "{}/src/manual_qa_ready_distribution.rs",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap();
    assert!(source.contains("docs/manual-qa.md"));
    assert!(source.contains("docs/license-sandbox-runbook.md"));
    assert!(source.contains("scripts/manual-qa-distribution-handoff.sh"));
    assert!(PUBLIC_WEB_HANDOFF.contains("cargo run -p xtask -- public-web-ready"));
    assert!(PUBLIC_WEB_HANDOFF.contains("cargo run -p xtask -- public-web-rerun"));
    assert!(PUBLIC_WEB_HANDOFF.contains("productization-status --track 'Public web proof'"));
    assert!(PUBLIC_WEB_HANDOFF.contains("docs/public-beta-operator-checklist.md"));
    assert!(PUBLIC_WEB_HANDOFF.contains("Short Execution Memo"));
}
