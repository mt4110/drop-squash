use std::path::{Path, PathBuf};

const SECRET_EXTENSIONS: [&str; 9] = [
    "cer",
    "cert",
    "crt",
    "key",
    "mobileprovision",
    "p12",
    "p8",
    "pem",
    "provisionprofile",
];

const LOCAL_EVIDENCE_EXTENSIONS: [&str; 7] = ["csv", "dmg", "jsonl", "m4v", "mov", "mp4", "webm"];

pub(super) fn reject_secret_files(root: &Path) -> Result<(), String> {
    for path in repo_files(root)? {
        let name = path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or("");
        let extension = path.extension().and_then(|value| value.to_str());
        if is_secret_file(name, extension) {
            return Err(format!(
                "release secret-like file is present: {}",
                path.display()
            ));
        }
        if is_local_evidence_file(extension) {
            return Err(format!(
                "release local evidence file must stay outside the repository: {}",
                path.display()
            ));
        }
    }
    Ok(())
}

pub(super) fn require_local_agent_ignore(path: &Path) -> Result<(), String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    require_ignore_line(&text, path, "/.codex/")?;
    require_ignore_line(&text, path, "/.direnv/")?;
    require_ignore_line(&text, path, "/result")?;
    require_ignore_line(&text, path, "/result-*")
}

fn require_ignore_line(text: &str, path: &Path, needle: &str) -> Result<(), String> {
    if text.lines().map(str::trim).any(|line| line == needle) {
        Ok(())
    } else {
        Err(format!("{} must ignore {needle}", path.display()))
    }
}

pub(super) fn is_secret_file(name: &str, extension: Option<&str>) -> bool {
    name == ".env"
        || name == ".envrc"
        || name.starts_with(".env.")
        || extension.is_some_and(|value| {
            let lower = value.to_ascii_lowercase();
            SECRET_EXTENSIONS.contains(&lower.as_str())
        })
}

fn is_local_evidence_file(extension: Option<&str>) -> bool {
    extension.is_some_and(|value| {
        let lower = value.to_ascii_lowercase();
        LOCAL_EVIDENCE_EXTENSIONS.contains(&lower.as_str())
    })
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
        .is_some_and(|name| {
            matches!(
                name,
                ".codex" | ".direnv" | ".git" | ".private_docs" | "target" | "node_modules"
            ) || name == "result"
                || name.starts_with("result-")
        })
}
