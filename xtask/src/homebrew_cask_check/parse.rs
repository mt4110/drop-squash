pub(super) struct Cask {
    pub(super) version: String,
    pub(super) url: String,
    pub(super) sha256: String,
}

pub(super) struct Notes {
    pub(super) version: String,
    pub(super) artifact_url: String,
    pub(super) sha256: String,
}

pub(super) fn cask(text: &str) -> Result<Cask, String> {
    require(text, "cask \"dropsquash\" do", "dropsquash cask name")?;
    require(text, "app \"DropSquash.app\"", "DropSquash app stanza")?;
    require(text, "auto_updates false", "auto_updates false")?;
    require(
        text,
        "zap trash: \"~/Library/Application Support/DropSquash\"",
        "DropSquash zap path",
    )?;
    Ok(Cask {
        version: quoted_value(text, "version")?,
        url: quoted_value(text, "url")?,
        sha256: quoted_value(text, "sha256")?,
    })
}

pub(super) fn notes(text: &str) -> Result<Notes, String> {
    let version = field(text, "Version")?;
    let Some(version) = version.strip_prefix('v') else {
        return Err("release notes Version must use v<version>".to_string());
    };
    Ok(Notes {
        version: version.to_string(),
        artifact_url: field(text, "Artifact URL")?,
        sha256: field(text, "SHA-256")?,
    })
}

fn require(text: &str, expected: &str, label: &str) -> Result<(), String> {
    if text.contains(expected) {
        return Ok(());
    }
    Err(format!("Homebrew cask must include {label}"))
}

fn quoted_value(text: &str, key: &str) -> Result<String, String> {
    let prefix = format!("{key} \"");
    let line = text
        .lines()
        .find(|line| line.trim_start().starts_with(&prefix))
        .ok_or_else(|| format!("Homebrew cask must include {key}"))?;
    let value = line
        .trim_start()
        .strip_prefix(&prefix)
        .and_then(|rest| rest.strip_suffix('"'))
        .ok_or_else(|| format!("Homebrew cask {key} must be quoted"))?;
    if value.is_empty() {
        return Err(format!("Homebrew cask {key} must not be empty"));
    }
    Ok(value.to_string())
}

fn field(text: &str, label: &str) -> Result<String, String> {
    let prefix = format!("- {label}: ");
    let line = text
        .lines()
        .find(|line| line.starts_with(&prefix))
        .ok_or_else(|| format!("release notes must include {label}"))?;
    let value = line
        .strip_prefix(&prefix)
        .expect("field prefix was checked")
        .trim();
    if value.is_empty() || value.starts_with("pending ") {
        return Err(format!("release notes {label} must be concrete"));
    }
    Ok(value.to_string())
}
