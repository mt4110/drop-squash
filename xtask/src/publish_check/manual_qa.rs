use std::path::Path;

pub(super) fn require_public_dmg(path: &Path) -> Result<(), String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read manual QA {}: {error}", path.display()))?;
    let Some(artifact) = app_artifact(&text) else {
        return Err("manual QA App artifact must be present before publish".to_string());
    };
    if Path::new(artifact)
        .file_name()
        .and_then(|name| name.to_str())
        == Some("DropSquash.dmg")
    {
        return Ok(());
    }
    Err("manual QA App artifact must be the public DropSquash.dmg before publish".to_string())
}

fn app_artifact(text: &str) -> Option<&str> {
    text.lines().find_map(|line| {
        let cells = line
            .trim()
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .collect::<Vec<_>>();
        (cells.len() == 2 && cells.first() == Some(&"App artifact")).then_some(cells[1])
    })
}

#[cfg(test)]
mod tests {
    use super::require_public_dmg;

    #[test]
    fn accepts_public_dmg_artifact() {
        let (_directory, path) = write_manual_qa("| App artifact | /tmp/DropSquash.dmg |\n");

        assert!(require_public_dmg(&path).is_ok());
    }

    #[test]
    fn rejects_app_artifact_before_publish() {
        let (_directory, path) = write_manual_qa("| App artifact | /tmp/DropSquash.app |\n");

        assert!(require_public_dmg(&path)
            .unwrap_err()
            .contains("public DropSquash.dmg"));
    }

    fn write_manual_qa(text: &str) -> (tempfile::TempDir, std::path::PathBuf) {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("manual-qa.md");
        std::fs::write(&path, text).unwrap();
        (directory, path)
    }
}
