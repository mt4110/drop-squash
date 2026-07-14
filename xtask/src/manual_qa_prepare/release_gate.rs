pub(super) fn rows() -> Vec<String> {
    vec![
        "| `cargo run -p xtask -- release-check` | Passes |  |".to_string(),
        "| `cargo run -p xtask -- file-size-check` | Passes |  |".to_string(),
        "| `cargo run -p xtask -- media-policy-check` | Passes |  |".to_string(),
        "| `cargo run -p xtask -- privacy-policy-check` | Passes |  |".to_string(),
        "| `cargo run -p xtask -- website-check` | Passes |  |".to_string(),
        "| `cargo run -p xtask -- manual-qa-check` | Passes after every manual QA result is recorded |  |".to_string(),
        "| `cargo run -p xtask -- homebrew-cask-check packaging/homebrew/Casks/dropsquash.rb path/to/release-notes.md` | Generated `dropsquash.rb` cask matches `.md` release notes version, Artifact URL, SHA-256, `auto_updates false`, and `zap` |  |".to_string(),
        "| `cargo run -p xtask -- macos-signing-check` | Passes in release environment |  |".to_string(),
        "| Codesign verification | Public DMG/app artifact verifies with Developer ID signature |  |".to_string(),
        "| Notarization staple verification | Public DMG/app artifact passes notary, stapler validate or stapled status, and `spctl` assessment |  |".to_string(),
        "| Gatekeeper open test | Signed, notarized, stapled app from public `DropSquash.dmg` matching the release notes Artifact URL opens cleanly without Gatekeeper warning |  |".to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::rows;

    #[test]
    fn generated_rows_match_required_manual_qa_checks() {
        let rows = rows().join("\n");

        for check in [
            "`cargo run -p xtask -- release-check`",
            "`cargo run -p xtask -- file-size-check`",
            "`cargo run -p xtask -- media-policy-check`",
            "`cargo run -p xtask -- privacy-policy-check`",
            "`cargo run -p xtask -- website-check`",
            "`cargo run -p xtask -- manual-qa-check`",
            "`cargo run -p xtask -- homebrew-cask-check packaging/homebrew/Casks/dropsquash.rb path/to/release-notes.md`",
            "`cargo run -p xtask -- macos-signing-check`",
            "Codesign verification",
            "Notarization staple verification",
            "Gatekeeper open test",
        ] {
            assert!(crate::manual_qa_check::requirements::REQUIRED_CHECKS.contains(&check));
            assert!(rows.contains(check));
        }
    }

    #[test]
    fn generated_rows_exist_in_manual_qa_template() {
        let template = std::fs::read_to_string("../docs/manual-qa.md").unwrap();

        for row in rows() {
            let check = row.split('|').nth(1).unwrap().trim();
            assert!(template.contains(check));
        }
    }
}
