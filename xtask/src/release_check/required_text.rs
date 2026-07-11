use std::fs;

const REQUIRED_TEXT: [(&str, &str); 26] = [
    ("docs/release.md", "docs/release-blockers.md"),
    ("docs/release-blockers.md", "must include the public URL"),
    ("docs/productization.md", "docs/release-blockers.md"),
    ("docs/productization.md", "final publish gate"),
    (
        "docs/productization.md",
        "refund/support contact copy gates",
    ),
    ("README.md", "macOS today"),
    ("README.md", "Windows and Linux support is planned"),
    (
        "Cargo.toml",
        "repository = \"https://github.com/mt4110/drop-squash\"",
    ),
    ("docs/product.md", "macOS today"),
    ("docs/product.md", "Windows and Linux support is planned"),
    ("docs/qa-evidence.md", "docs/release-blockers.md"),
    (
        "docs/qa-evidence.md",
        "license, refund, support contact copy",
    ),
    ("docs/qa-evidence.md", "file names"),
    ("docs/qa-evidence.md", "UDIF `.dmg` artifacts"),
    ("docs/qa-evidence.md", "--restore-state"),
    ("docs/qa-evidence.md", "non-DMG targets are rejected"),
    ("docs/qa-evidence.md", "publish-check"),
    ("docs/qa-evidence.md", "release-notes-check"),
    (
        "docs/qa-evidence.md",
        "post-encode postprocess/history guard",
    ),
    ("docs/qa-evidence.md", "UDIF trailer"),
    ("docs/qa-evidence.md", "non-canonical homepages"),
    ("docs/licensing.md", "Lemon Squeezy sandbox purchase"),
    (
        "docs/benchmarking.md",
        "at least three private local samples",
    ),
    ("docs/benchmarking.md", "20%"),
    ("website/README.md", "docs/release-blockers.md"),
    ("apps/desktop/src-tauri/tauri.conf.json", "!\"updater\""),
];

pub(super) fn check() -> Result<(), String> {
    for (path, needle) in REQUIRED_TEXT {
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
