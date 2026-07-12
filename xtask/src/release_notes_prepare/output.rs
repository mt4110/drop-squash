pub(super) struct Fields<'a> {
    pub(super) version: &'a str,
    pub(super) artifact_path: &'a str,
    pub(super) artifact_url: &'a str,
    pub(super) sha256: &'a str,
    pub(super) commit: &'a str,
}

pub(super) fn lines(fields: Fields<'_>) -> Vec<String> {
    let release_url = format!(
        "https://github.com/mt4110/drop-squash/releases/tag/v{}",
        fields.version
    );
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
        "SHA256SUMS output command:".into(),
        format!(
            "cargo run -p xtask -- checksum {} --output SHA256SUMS",
            shell_arg(fields.artifact_path)
        ),
        format!(
            "- GitHub Release checksum: pending upload; after attaching SHA256SUMS to {release_url} for {} with {}, replace this line with public release evidence",
            fields.artifact_url, fields.sha256
        ),
        format!("- GitHub Release URL: {release_url}"),
        "Homebrew cask command:".into(),
        format!(
            "cargo run -p xtask -- homebrew-cask {} {} {} https://github.com/mt4110/drop-squash",
            fields.version, fields.artifact_url, fields.sha256
        ),
        "Homebrew tap PR evidence draft:".into(),
        format!(
            "- Homebrew tap PR: cask PR for versioned DropSquash.dmg uses {} with SHA-256 {}, auto_updates false, and zap cleanup path",
            fields.artifact_url, fields.sha256
        ),
    ]
}

fn shell_arg(value: &str) -> String {
    if value
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || "/._-".contains(ch))
    {
        return value.to_string();
    }
    format!("'{}'", value.replace('\'', "'\\''"))
}
