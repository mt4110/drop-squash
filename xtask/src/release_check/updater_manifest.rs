use std::path::Path;

pub(super) fn reject_committed_manifests(root: &Path) -> Result<(), String> {
    let mut found = Vec::new();
    collect(root, &mut found)?;
    if found.is_empty() {
        return Ok(());
    }
    Err(format!(
        "updater is disabled; remove committed update manifests: {}",
        found.join(", ")
    ))
}

fn collect(path: &Path, found: &mut Vec<String>) -> Result<(), String> {
    if should_skip(path) {
        return Ok(());
    }
    if path.is_dir() {
        for entry in std::fs::read_dir(path).map_err(|error| error.to_string())? {
            collect(&entry.map_err(|error| error.to_string())?.path(), found)?;
        }
        return Ok(());
    }
    if is_update_manifest(path) {
        found.push(path.display().to_string());
    }
    Ok(())
}

fn is_update_manifest(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
        return false;
    };
    let lower = name.to_ascii_lowercase();
    lower == "latest.json" || lower.contains("update-manifest")
}

fn should_skip(path: &Path) -> bool {
    path.file_name()
        .and_then(|value| value.to_str())
        .is_some_and(|name| matches!(name, ".git" | ".codex" | "target" | "node_modules"))
}

#[cfg(test)]
mod tests {
    use super::reject_committed_manifests;

    #[test]
    fn accepts_tree_without_update_manifest() {
        let directory = tempfile::tempdir().unwrap();
        std::fs::write(directory.path().join("release.json"), "{}").unwrap();

        reject_committed_manifests(directory.path()).unwrap();
    }

    #[test]
    fn rejects_latest_json() {
        let directory = tempfile::tempdir().unwrap();
        std::fs::write(directory.path().join("latest.json"), "{}").unwrap();

        let error = reject_committed_manifests(directory.path()).unwrap_err();

        assert!(error.contains("latest.json"));
    }

    #[test]
    fn rejects_update_manifest_name() {
        let directory = tempfile::tempdir().unwrap();
        std::fs::write(directory.path().join("drop-update-manifest.json"), "{}").unwrap();

        let error = reject_committed_manifests(directory.path()).unwrap_err();

        assert!(error.contains("update-manifest"));
    }
}
