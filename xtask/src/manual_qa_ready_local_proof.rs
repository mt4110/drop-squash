mod commands;
mod samples;

use std::path::{Path, PathBuf};

#[cfg(test)]
mod tests;

const DRAFT_MARKER: &str = "Prepared manual QA draft only.";
const USAGE: &str = "usage: cargo run -p xtask -- manual-qa-ready-local-proof <manual-qa.md> <results.csv> [baseline-results.csv]";
const PACKAGED_GUIDE: &str = "docs/manual-qa.md";
const LICENSE_RUNBOOK: &str = "docs/license-sandbox-runbook.md";
const SIGNED_DMG_RUNBOOK: &str = "docs/signed-dmg-runbook.md";
const DISTRIBUTION_HANDOFF: &str =
    "distribution snapshot handoff: scripts/manual-qa-distribution-handoff.sh /tmp/dropsquash-qa-snapshot-$(git rev-parse --short HEAD)";

use commands::{
    fresh_artifact_path, fresh_build_command, fresh_not_smaller_command,
    installed_app_restore_command, installed_app_stash_command, installed_app_status_command,
    manual_check_command, packaged_app_pending_command, sample_link_command,
};
use samples::sample_hints;

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let parsed = parse_args(args)?;
    crate::manual_qa_fill_local_proof::run(run_args(&parsed))?;
    finish(&parsed.0)?;
    let manual_text = std::fs::read_to_string(&parsed.0).map_err(|error| error.to_string())?;
    println!(
        "fresh packaged-app build command: {}",
        fresh_build_command()
    );
    println!("fresh packaged-app artifact: {}", fresh_artifact_path());
    println!(
        "prepared local proof draft is ready: {}",
        parsed.0.display()
    );
    println!("packaged manual QA guide: {PACKAGED_GUIDE}");
    println!("license sandbox runbook: {LICENSE_RUNBOOK}");
    println!("signed DMG runbook: {SIGNED_DMG_RUNBOOK}");
    println!("{DISTRIBUTION_HANDOFF}");
    println!(
        "next packaged-app pending command: {}",
        packaged_app_pending_command(&parsed.0)
    );
    println!(
        "{}",
        crate::manual_qa_observation::packaged_visibility_reminder(
            "packaged-app observation reminder",
        )
    );
    println!(
        "fresh packaged-app sample-link command: {}",
        sample_link_command(&parsed.1)
    );
    println!(
        "fresh packaged-app installed-app status: {}",
        installed_app_status_command()
    );
    println!(
        "fresh packaged-app installed-app stash: {}",
        installed_app_stash_command()
    );
    println!(
        "fresh packaged-app installed-app restore: {}",
        installed_app_restore_command()
    );
    println!("next paid beta check command: cargo run -p xtask -- paid-beta-check");
    println!(
        "next manual QA check command: {}",
        manual_check_command(&parsed.0)
    );
    for line in sample_hints(&parsed.1)? {
        println!("{line}");
    }
    if let Some(command) = fresh_not_smaller_command(&manual_text, &parsed.1)? {
        println!("fresh packaged-app not-smaller command: {command}");
    }
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<(PathBuf, PathBuf, Option<PathBuf>), String> {
    match args.as_slice() {
        [manual, current] if manual != "--help" && manual != "-h" => {
            Ok((PathBuf::from(manual), PathBuf::from(current), None))
        }
        [manual, current, baseline] if manual != "--help" && manual != "-h" => Ok((
            PathBuf::from(manual),
            PathBuf::from(current),
            Some(PathBuf::from(baseline)),
        )),
        _ => Err(USAGE.to_string()),
    }
}

fn run_args(parsed: &(PathBuf, PathBuf, Option<PathBuf>)) -> Vec<String> {
    let mut args = vec![display(&parsed.0), display(&parsed.1)];
    if let Some(path) = parsed.2.as_ref() {
        args.push(display(path));
    }
    args
}

fn finish(path: &Path) -> Result<(), String> {
    if needs_cleaning(path)? {
        crate::manual_qa_clean_draft::run(vec![display(path)])?;
    }
    Ok(())
}

fn needs_cleaning(path: &Path) -> Result<bool, String> {
    Ok(std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read manual QA file: {error}"))?
        .contains(DRAFT_MARKER))
}

fn display(path: &Path) -> String {
    path.display().to_string()
}

fn shell_single_quote(path: &Path) -> String {
    path.display().to_string().replace('\'', "'\\''")
}
