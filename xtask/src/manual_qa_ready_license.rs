mod state;
mod steps;

use std::path::PathBuf;

#[cfg(test)]
mod tests;

const USAGE: &str = "usage: cargo run -p xtask -- manual-qa-ready-license <manual-qa.md>";

use state::{
    is_tmp_path, isolation_recovery_line, license_cache_path, license_pending_command,
    manual_check_command, sandbox_quickstart_lines, sandbox_rows_command, uses_dmg_artifact,
};

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let path = parse_args(args)?;
    crate::manual_qa_release_gate_fill::run(vec![path.display().to_string()])?;
    finish(&path)?;
    let text = std::fs::read_to_string(&path)
        .map_err(|error| format!("failed to read manual QA file: {error}"))?;
    println!("license proof draft is ready: {}", path.display());
    steps::print_intro();
    if uses_dmg_artifact(&text) {
        println!(
            "license launch note: App artifact is a DMG; prefer the fresh app launch commands below for sandbox activation loops and use the mounted DMG launch only when you intentionally need mounted-DMG behavior"
        );
    }
    println!(
        "next license pending command: {}",
        license_pending_command(&path)
    );
    println!(
        "next sandbox markdown rows command: {}",
        sandbox_rows_command(&path)
    );
    println!("next sandbox row candidates:");
    for line in crate::manual_qa_pending::ready::license_sandbox_rows() {
        println!("{line}");
    }
    for line in sandbox_quickstart_lines(&text) {
        println!("{line}");
    }
    for line in crate::manual_qa_pending::ready::license_ready_lines(&text) {
        println!("{line}");
    }
    if let Some(cache_path) = license_cache_path(&text) {
        if !is_tmp_path(&cache_path) {
            println!(
                "license isolation note: cache path is not under /tmp; prefer a prepared manual QA file with isolated app state before sandbox activation"
            );
            if let Some(line) = isolation_recovery_line(&cache_path) {
                println!("{line}");
            }
        }
    }
    steps::print_next_steps();
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

fn needs_cleaning(path: &PathBuf) -> Result<bool, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read manual QA file: {error}"))?;
    Ok(text.contains(steps::DRAFT_MARKER))
}

fn finish(path: &PathBuf) -> Result<(), String> {
    if needs_cleaning(path)? {
        crate::manual_qa_clean_draft::run(vec![path.display().to_string()])?;
    }
    Ok(())
}
