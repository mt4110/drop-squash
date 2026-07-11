use crate::{media_policy_check, privacy_policy_check};

use std::path::{Path, PathBuf};

const RELEASE_WORKFLOW: &str = ".github/workflows/release.yml";
const RELEASE_WORKFLOW_GATES: [&str; 8] = [
    "cargo run -p xtask -- file-size-check",
    "cargo run -p xtask -- website-check",
    "cargo run -p xtask -- release-check",
    "pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci",
    "cargo run -p xtask -- artifact-check target/release/bundle/dmg/*.dmg",
    "cargo run -p xtask -- checksum target/release/bundle/dmg/*.dmg > SHA256SUMS",
    "cargo run -p xtask -- macos-signing-check",
    "Block unsigned Phase 0 release",
];
const SECRET_EXTENSIONS: [&str; 6] = [
    "key",
    "mobileprovision",
    "p12",
    "p8",
    "pem",
    "provisionprofile",
];

pub fn run() -> Result<(), String> {
    reject_secret_files(Path::new("."))?;
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

fn reject_secret_files(root: &Path) -> Result<(), String> {
    for path in repo_files(root)? {
        let name = path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or("");
        if is_secret_file(name, path.extension().and_then(|value| value.to_str())) {
            return Err(format!(
                "release secret-like file is present: {}",
                path.display()
            ));
        }
    }
    Ok(())
}

fn is_secret_file(name: &str, extension: Option<&str>) -> bool {
    name == ".env"
        || name.starts_with(".env.")
        || extension.is_some_and(|value| {
            let lower = value.to_ascii_lowercase();
            SECRET_EXTENSIONS.contains(&lower.as_str())
        })
}

#[cfg(test)]
mod tests;

fn repo_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    collect_files(root, &mut files)?;
    Ok(files)
}

fn collect_files(path: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in std::fs::read_dir(path).map_err(|error| error.to_string())? {
        let path = entry.map_err(|error| error.to_string())?.path();
        if should_skip(&path) {
            continue;
        }
        if path.is_dir() {
            collect_files(&path, files)?;
        } else {
            files.push(path);
        }
    }
    Ok(())
}

fn should_skip(path: &Path) -> bool {
    path.file_name()
        .and_then(|value| value.to_str())
        .is_some_and(|name| matches!(name, ".git" | ".private_docs" | "target" | "node_modules"))
}

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
