pub(super) struct Fields<'a> {
    pub(super) version: &'a str,
    pub(super) artifact_url: &'a str,
    pub(super) sha256: &'a str,
    pub(super) commit: &'a str,
}

pub(super) fn lines(fields: Fields<'_>) -> Vec<String> {
    vec![
        "release notes prepared fields:".into(),
        "## Artifact".into(),
        format!("- Version: v{}", fields.version),
        "- Artifact: DropSquash.dmg".into(),
        format!("- Artifact URL: {}", fields.artifact_url),
        format!("- SHA-256: {}", fields.sha256),
        format!("- Git commit: {}", fields.commit),
        "## Distribution".into(),
        format!("- SHA256SUMS line: {}  DropSquash.dmg", fields.sha256),
        format!(
            "- GitHub Release checksum: pending upload; after attaching SHA256SUMS for {} with {}, replace this line with public release evidence",
            fields.artifact_url, fields.sha256
        ),
        format!(
            "- GitHub Release URL: https://github.com/mt4110/drop-squash/releases/tag/v{}",
            fields.version
        ),
        "Homebrew cask command:".into(),
        format!(
            "cargo run -p xtask -- homebrew-cask {} {} {} https://github.com/mt4110/drop-squash",
            fields.version, fields.artifact_url, fields.sha256
        ),
    ]
}
