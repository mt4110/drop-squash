use std::path::{Path, PathBuf};

const DISALLOWED_VERSION_MANAGER_FILES: [&str; 3] = [".mise.toml", "mise.toml", ".tool-versions"];

pub(super) fn reject_parallel_version_manager(root: &Path) -> Result<(), String> {
    for path in repo_files(root)? {
        let name = path.file_name().and_then(|value| value.to_str());
        if name.is_some_and(|value| DISALLOWED_VERSION_MANAGER_FILES.contains(&value)) {
            return Err(format!(
                "release dev environment must use Nix without parallel version manager config: {}",
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
        .is_some_and(|name| {
            matches!(
                name,
                ".codex" | ".direnv" | ".git" | ".private_docs" | "target" | "node_modules"
            ) || name.starts_with("result")
        })
}
