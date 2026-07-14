use std::path::Path;

pub(super) struct Fields<'a> {
    pub(super) version: &'a str,
    pub(super) artifact_path: &'a str,
    pub(super) artifact_url: &'a str,
    pub(super) sha256: &'a str,
    pub(super) release_url: &'a str,
}

pub(super) fn lines(fields: Fields<'_>) -> Vec<String> {
    let checksum_path = checksum_path(fields.artifact_path);
    let cask_path = "packaging/homebrew/Casks/dropsquash.rb";
    vec![
        "## Distribution".into(),
        format!("- SHA256SUMS line: {}  DropSquash.dmg", fields.sha256),
        "SHA256SUMS output command:".into(),
        format!(
            "cargo run -p xtask -- checksum {} --output {}",
            shell_arg(fields.artifact_path),
            shell_arg(&checksum_path)
        ),
        format!(
            "- GitHub Release checksum: pending upload; after attaching SHA256SUMS to public {} for the exact Artifact URL {} with lowercase SHA-256 {}, replace this line with public release evidence that includes the GitHub Release URL above and the Artifact URL above",
            fields.release_url, fields.artifact_url, fields.sha256
        ),
        format!("- GitHub Release URL: {}", fields.release_url),
        "GitHub Release command plan:".into(),
        format!(
            "cargo run -p xtask -- github-release-plan v{} {} {} /tmp/dropsquash-release-notes.md",
            fields.version,
            shell_arg(fields.artifact_path),
            shell_arg(&checksum_path)
        ),
        "Homebrew cask command:".into(),
        format!(
            "cargo run -p xtask -- homebrew-cask {} {} {} https://github.com/mt4110/drop-squash > {}",
            fields.version,
            shell_arg(fields.artifact_url),
            fields.sha256,
            shell_arg(cask_path)
        ),
        "Homebrew cask check command:".into(),
        format!(
            "cargo run -p xtask -- homebrew-cask-check {} /tmp/dropsquash-release-notes.md",
            shell_arg(cask_path)
        ),
        "Homebrew tap PR evidence draft:".into(),
        "- Homebrew tap PR URL: pending tap PR; replace this line with the reviewed Homebrew tap PR URL".into(),
        format!(
            "- Homebrew tap PR: public cask PR for versioned DropSquash.dmg uses the Artifact URL above {} with lowercase SHA-256 {}, auto_updates false, and zap cleanup path; replace this line with reviewed public PR evidence that includes the Homebrew tap PR URL above",
            fields.artifact_url, fields.sha256
        ),
        "Homebrew install result evidence draft:".into(),
        format!(
            "- Homebrew install result: after `brew install --cask mt4110/tap/dropsquash` from the Homebrew tap PR URL above installs the versioned DropSquash.dmg artifact from the Artifact URL above {} with lowercase SHA-256 {} and `brew uninstall --cask mt4110/tap/dropsquash` removes it cleanly, replace this line with observed install and uninstall evidence",
            fields.artifact_url, fields.sha256
        ),
    ]
}

fn checksum_path(artifact_path: &str) -> String {
    Path::new(artifact_path)
        .with_file_name("SHA256SUMS")
        .display()
        .to_string()
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
