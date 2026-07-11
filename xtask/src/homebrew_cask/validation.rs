pub(super) fn require_version(version: &str) -> Result<(), String> {
    require_clean("version", version)?;
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() == 3 && parts.iter().all(|part| is_numeric_part(part)) {
        return Ok(());
    }
    Err("version must use major.minor.patch digits".to_string())
}

pub(super) fn require_dmg_url(url: &str) -> Result<(), String> {
    require_https_url(url)?;
    require_github_release_url(url)?;
    if url.ends_with(".dmg") {
        return Ok(());
    }
    Err("url must point to a .dmg file".to_string())
}

pub(super) fn require_versioned_url(version: &str, url: &str) -> Result<(), String> {
    if url.contains(&format!("/v{version}/")) {
        return Ok(());
    }
    Err("url must point to the matching v<version> release".to_string())
}

pub(super) fn require_https_url(url: &str) -> Result<(), String> {
    require_clean("url", url)?;
    if url.contains("example.com") {
        return Err("url must not contain example.com".to_string());
    }
    if url.starts_with("https://") {
        return Ok(());
    }
    Err("url must start with https://".to_string())
}

pub(super) fn require_sha256(value: &str) -> Result<(), String> {
    require_clean("sha256", value)?;
    if value.len() == 64 && value.chars().all(|char| char.is_ascii_hexdigit()) {
        return Ok(());
    }
    Err("sha256 must be 64 hex characters".to_string())
}

fn require_clean(label: &str, value: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.chars().any(char::is_whitespace) {
        return Err(format!(
            "{label} must be non-empty and contain no whitespace"
        ));
    }
    Ok(())
}

fn require_github_release_url(url: &str) -> Result<(), String> {
    if url.starts_with("https://github.com/mt4110/drop-squash/releases/download/") {
        return Ok(());
    }
    Err("url must point to the DropSquash GitHub Release download".to_string())
}

fn is_numeric_part(value: &str) -> bool {
    !value.is_empty() && value.chars().all(|char| char.is_ascii_digit())
}
