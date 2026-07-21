use std::path::{Path, PathBuf};

mod output;

use output::{distribution_quickstart, uses_local_unsigned_dmg};
#[cfg(test)]
mod tests;

const DRAFT_MARKER: &str = "Prepared manual QA draft only.";
const QUICKSTART_NOTE: &str =
    "distribution quickstart note: after quickstart 4, run the emitted signing-plan steps so /tmp/dropsquash-signed-release/DropSquash.dmg exists before quickstart 5..7";
const SIGNING_ENV_CHECK: &str =
    "distribution signing environment check: CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- macos-signing-check";
const SIGNING_ENV_NOTE: &str =
    "distribution signing environment note: local macOS needs APPLE_SIGNING_IDENTITY or APPLE_CERTIFICATE with APPLE_CERTIFICATE_PASSWORD; notarization needs APPLE_API_KEY/APPLE_API_ISSUER/APPLE_API_KEY_PATH or APPLE_ID/APPLE_PASSWORD/APPLE_TEAM_ID; CI signing also needs APPLE_KEYCHAIN_PASSWORD and APPLE_CODESIGN_IDENTITY";
const PAID_BETA_CHECKLIST: &str = "docs/paid-beta-operator-checklist.md";
const DISTRIBUTION_HANDOFF: &str =
    "distribution snapshot handoff: scripts/manual-qa-distribution-handoff.sh /tmp/dropsquash-qa-snapshot-$(git rev-parse --short HEAD)";
const READY_ALL_HINT: &str =
    "distribution deterministic helper: if you have a fresh prepared manual-QA draft and checked benchmark CSV, run `cargo run -p xtask -- manual-qa-ready-all <manual-qa.md> <results.csv>` before this rerun";
const PUBLIC_WEB_HANDOFF: &str =
    "after signing proof: start with `cargo run -p xtask -- public-web-ready`, use `cargo run -p xtask -- public-web-rerun` as the operator memo when needed, then run `cargo run -p xtask -- productization-status --track 'Public web proof'` and follow `docs/public-beta-operator-checklist.md` (`Short Execution Memo` for the fast path) before production onboarding";
const USAGE: &str = "usage: cargo run -p xtask -- manual-qa-ready-distribution <manual-qa.md>";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let path = parse_args(args)?;
    crate::manual_qa_release_gate_fill::run(vec![display(&path)])?;
    finish(&path)?;
    let text = std::fs::read_to_string(&path)
        .map_err(|error| format!("failed to read manual QA file: {error}"))?;
    println!("distribution proof draft is ready: {}", path.display());
    println!("packaged manual QA guide: docs/manual-qa.md");
    println!("license sandbox runbook: docs/license-sandbox-runbook.md");
    println!("next signed DMG runbook: docs/signed-dmg-runbook.md");
    println!("distribution operator checklist: {PAID_BETA_CHECKLIST}");
    println!("{DISTRIBUTION_HANDOFF}");
    println!("{READY_ALL_HINT}");
    if uses_local_unsigned_dmg(&text) {
        println!(
            "distribution artifact note: App artifact is the local unsigned QA DMG; use the printed plan/check commands as preflight, then record final rows against the signed public DropSquash.dmg and its public Artifact URL"
        );
    }
    println!(
        "next distribution pending command: {}",
        distribution_pending_command(&path)
    );
    println!("{SIGNING_ENV_CHECK}");
    println!("{SIGNING_ENV_NOTE}");
    for line in distribution_quickstart(&text) {
        println!("{line}");
    }
    println!("{QUICKSTART_NOTE}");
    if let Ok(rows) = crate::manual_qa_pending::markdown_rows(&path, "distribution") {
        if !rows.is_empty() {
            println!("pending distribution markdown rows:\n{}", rows.join("\n"));
        }
    }
    for line in crate::manual_qa_pending::ready::distribution_lines(&text) {
        println!("{line}");
    }
    println!("{PUBLIC_WEB_HANDOFF}");
    println!("next paid beta check command: cargo run -p xtask -- paid-beta-check");
    println!(
        "next manual QA check command: {}",
        manual_check_command(&path)
    );
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<PathBuf, String> {
    match args.as_slice() {
        [path] if path != "--help" && path != "-h" => Ok(PathBuf::from(path)),
        _ => Err(USAGE.to_string()),
    }
}

fn needs_cleaning(path: &Path) -> Result<bool, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read manual QA file: {error}"))?;
    Ok(text.contains(DRAFT_MARKER))
}

fn finish(path: &Path) -> Result<(), String> {
    if needs_cleaning(path)? {
        crate::manual_qa_clean_draft::run(vec![display(path)])?;
    }
    Ok(())
}

fn display(path: &Path) -> String {
    path.display().to_string()
}

fn distribution_pending_command(path: &Path) -> String {
    format!(
        "cargo run -p xtask -- manual-qa-pending '{}' --section distribution",
        shell_single_quote(path)
    )
}

fn manual_check_command(path: &Path) -> String {
    format!(
        "cargo run -p xtask -- manual-qa-check '{}'",
        shell_single_quote(path)
    )
}

fn shell_single_quote(path: &Path) -> String {
    path.display().to_string().replace('\'', "'\\''")
}
