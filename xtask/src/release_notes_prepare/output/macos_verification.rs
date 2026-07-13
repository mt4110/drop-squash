use super::shell_arg;

pub(super) fn lines(artifact_path: &str, artifact_url: &str) -> Vec<String> {
    vec![
        "## macOS Verification".into(),
        "macOS verification commands:".into(),
        format!(
            "codesign --verify --deep --strict --verbose=2 {}",
            shell_arg(artifact_path)
        ),
        format!(
            "spctl --assess --type open --verbose=4 {}",
            shell_arg(artifact_path)
        ),
        format!("xcrun stapler validate {}", shell_arg(artifact_path)),
        format!("- `codesign`: pending Developer ID verification for public {artifact_url}; replace this line with observed `codesign` evidence that includes the exact Artifact URL"),
        format!("- `spctl`: pending Gatekeeper assessment for public {artifact_url}; replace this line with observed `spctl` accepted evidence that includes the exact Artifact URL"),
        format!("- `stapler`: pending stapled ticket validation for public {artifact_url}; replace this line with observed `stapler` validate evidence that includes the exact Artifact URL"),
        format!("- Apple notary log: pending notarytool accepted log for public {artifact_url}; replace this line with observed notary evidence that includes the exact Artifact URL"),
        format!("- Gatekeeper clean-machine open: pending clean-machine open test for public {artifact_url}; replace this line with observed Gatekeeper evidence that includes signed, notarized, stapled, and no warning"),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn generated_macos_commands_exist_in_release_notes_template() {
        let template = std::fs::read_to_string("../docs/release-notes-template.md").unwrap();
        for command in [
            "codesign --verify --deep --strict --verbose=2",
            "spctl --assess --type open --verbose=4",
            "xcrun stapler validate",
        ] {
            assert!(template.contains(command), "{command}");
        }
    }
}
