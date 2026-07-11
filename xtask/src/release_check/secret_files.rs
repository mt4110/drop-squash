use std::path::{Path, PathBuf};

const SECRET_EXTENSIONS: [&str; 6] = [
    "key",
    "mobileprovision",
    "p12",
    "p8",
    "pem",
    "provisionprofile",
];

pub(super) fn reject_secret_files(root: &Path) -> Result<(), String> {
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

pub(super) fn is_secret_file(name: &str, extension: Option<&str>) -> bool {
    name == ".env"
        || name.starts_with(".env.")
        || extension.is_some_and(|value| {
            let lower = value.to_ascii_lowercase();
            SECRET_EXTENSIONS.contains(&lower.as_str())
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
        .is_some_and(|name| matches!(name, ".git" | ".private_docs" | "target" | "node_modules"))
}
