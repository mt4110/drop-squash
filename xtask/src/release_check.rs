use std::path::{Path, PathBuf};

pub fn run() -> Result<(), String> {
    reject_secret_files(Path::new("."))?;
    require_text(
        ".github/workflows/release.yml",
        "Block unsigned Phase 0 release",
    )?;
    require_text(
        ".github/workflows/security.yml",
        "ffmpeg|ffprobe|std::process::Command|tokio::process",
    )?;
    require_text(
        "apps/desktop/src-tauri/tauri.conf.json",
        "\"connect-src\": \"ipc: http://ipc.localhost\"",
    )?;
    reject_text("apps/desktop/src-tauri/tauri.conf.json", "\"updater\"")?;
    println!("release readiness checks passed");
    Ok(())
}

fn reject_secret_files(root: &Path) -> Result<(), String> {
    for path in repo_files(root)? {
        let name = path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or("");
        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("");
        if matches!(name, ".env" | ".env.local") || matches!(extension, "p12" | "pem" | "key") {
            return Err(format!(
                "release secret-like file is present: {}",
                path.display()
            ));
        }
    }
    Ok(())
}

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
