use crate::{media_policy_check, privacy_policy_check};
mod secret_files;

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
    media_policy_check::check_default_roots()?;
    privacy_policy_check::check_default_roots()?;
    require_release_workflow_gates()?;
    require_text(
        "apps/desktop/src-tauri/tauri.conf.json",
        "\"connect-src\": \"ipc: http://ipc.localhost\"",
    )?;
    reject_text("apps/desktop/src-tauri/tauri.conf.json", "\"updater\"")?;
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

fn require_text(path: &str, needle: &str) -> Result<(), String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    if text.contains(needle) {
        return Ok(());
    }
    Err(format!("{path} is missing required text: {needle}"))
}

fn reject_text(path: &str, needle: &str) -> Result<(), String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    if text.contains(needle) {
        return Err(format!("{path} contains disallowed text: {needle}"));
    }
    Ok(())
}
