use std::path::Path;
use std::process::Command;

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

pub(super) fn require_current_head(path: &Path) -> Result<(), String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read manual QA {}: {error}", path.display()))?;
    let Some(build) = field_value(&text, "App build") else {
        return Err("manual QA App build must be present before publish".to_string());
    };
    let head = git_head()?;
    if build.contains(&format!("git {head}")) {
        return Ok(());
    }
    Err("manual QA App build must match current HEAD before publish".to_string())
}

fn app_artifact(text: &str) -> Option<&str> {
    field_value(text, "App artifact")
}

fn field_value<'a>(text: &'a str, label: &str) -> Option<&'a str> {
    text.lines().find_map(|line| {
        let cells = line
            .trim()
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .collect::<Vec<_>>();
        (cells.len() == 2 && cells.first() == Some(&label)).then_some(cells[1])
    })
}

fn git_head() -> Result<String, String> {
    let output = Command::new("git")
        .args(["rev-parse", "--short=7", "HEAD"])
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err("git rev-parse failed".to_string());
    }
    String::from_utf8(output.stdout)
        .map_err(|error| error.to_string())
        .map(|value| value.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::{require_current_head, require_public_dmg};

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

    #[test]
    fn accepts_manual_qa_for_current_head() {
        let head = super::git_head().unwrap();
        let (_directory, path) =
            write_manual_qa(&format!("| App build | DropSquash 0.1.0 git {head} |\n"));

        assert!(require_current_head(&path).is_ok());
    }

    #[test]
    fn rejects_manual_qa_for_old_head() {
        let (_directory, path) = write_manual_qa("| App build | DropSquash 0.1.0 git 0000000 |\n");

        assert!(require_current_head(&path)
            .unwrap_err()
            .contains("current HEAD"));
    }

    fn write_manual_qa(text: &str) -> (tempfile::TempDir, std::path::PathBuf) {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("manual-qa.md");
        std::fs::write(&path, text).unwrap();
        (directory, path)
    }
}
