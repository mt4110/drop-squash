use std::fs;

mod entries;

pub(super) fn check() -> Result<(), String> {
    for (path, needle) in entries::REQUIRED_TEXT {
        if let Some(rejected) = needle.strip_prefix('!') {
            reject_text(path, rejected)?;
        } else {
            require_text(path, needle)?;
        }
    }
    Ok(())
}

fn require_text(path: &str, needle: &str) -> Result<(), String> {
    let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
    if text.contains(needle) {
        return Ok(());
    }
    Err(format!("{path} is missing required text: {needle}"))
}

fn reject_text(path: &str, needle: &str) -> Result<(), String> {
    let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
    if text.contains(needle) {
        return Err(format!("{path} contains disallowed text: {needle}"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{reject_text, require_text};

    #[test]
    fn reports_missing_required_text() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("doc.md");
        std::fs::write(&path, "DropSquash").unwrap();

        let error = require_text(path.to_str().unwrap(), "release-blockers").unwrap_err();

        assert!(error.contains("release-blockers"));
    }

    #[test]
    fn reports_disallowed_text() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("tauri.conf.json");
        std::fs::write(&path, "\"updater\"").unwrap();

        let error = reject_text(path.to_str().unwrap(), "\"updater\"").unwrap_err();

        assert!(error.contains("disallowed text"));
    }
}
