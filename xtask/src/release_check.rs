use crate::{media_policy_check, privacy_policy_check};
mod blockers;
mod desktop_capability;
mod evidence;
mod manual_blockers;
mod release_doc;
mod release_notes;
mod required_text;
mod secret_files;
mod tauri_config;

use std::path::Path;

const RELEASE_WORKFLOW: &str = ".github/workflows/release.yml";
const RELEASE_WORKFLOW_GATES: [&str; 15] = [
    "components: rustfmt, clippy",
    "cargo fmt --all -- --check",
    "cargo clippy --workspace --all-targets -- -D warnings",
    "cargo test --workspace",
    "cargo run -p xtask -- file-size-check",
    "cargo run -p xtask -- website-check",
    "cargo run -p xtask -- manual-qa-check",
    "cargo run -p xtask -- release-check",
    "pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci",
    "cargo run -p xtask -- artifact-check target/release/bundle/dmg/*.dmg",
    "cargo run -p xtask -- checksum target/release/bundle/dmg/*.dmg > SHA256SUMS",
    "actions/upload-artifact@v4",
    "dropsquash-unsigned-dmg-checksum",
    "cargo run -p xtask -- macos-signing-check",
    "Block unsigned Phase 0 release",
];
pub fn run() -> Result<(), String> {
    secret_files::reject_secret_files(Path::new("."))?;
    desktop_capability::check_default_capability(Path::new(
        "apps/desktop/src-tauri/capabilities/default.json",
    ))?;
    media_policy_check::check_default_roots()?;
    privacy_policy_check::check_default_roots()?;
    evidence::check_manual_only_coverage(
        Path::new("docs/qa-evidence.md"),
        Path::new("docs/manual-qa.md"),
    )?;
    blockers::check_release_blockers(Path::new("docs/release-blockers.md"))?;
    manual_blockers::check(
        Path::new("docs/release-blockers.md"),
        Path::new("docs/manual-qa.md"),
    )?;
    release_doc::check(Path::new("docs/release.md"))?;
    release_notes::check(Path::new("docs/release-notes-template.md"))?;
    require_release_workflow_gates()?;
    tauri_config::check(Path::new("apps/desktop/src-tauri/tauri.conf.json"))?;
    required_text::check()?;
    println!("release readiness checks passed");
    Ok(())
}

fn require_release_workflow_gates() -> Result<(), String> {
    let text = std::fs::read_to_string(RELEASE_WORKFLOW).map_err(|error| error.to_string())?;
    let missing = missing_release_workflow_gates(&text);
    if missing.is_empty() {
        return Ok(());
    }
    Err(format!(
        "{RELEASE_WORKFLOW} is missing release gates: {}",
        missing.join(", ")
    ))
}

fn missing_release_workflow_gates(text: &str) -> Vec<&'static str> {
    RELEASE_WORKFLOW_GATES
        .iter()
        .copied()
        .filter(|gate| !text.contains(gate))
        .collect()
}

#[cfg(test)]
mod tests;
