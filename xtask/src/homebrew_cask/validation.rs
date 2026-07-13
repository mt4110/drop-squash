pub(super) fn require_version(version: &str) -> Result<(), String> {
    require_clean("version", version)?;
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() == 3 && parts.iter().all(|part| is_numeric_part(part)) {
        return Ok(());
    }
    Err("version must use major.minor.patch digits".to_string())
}

pub(super) fn require_dmg_url(url: &str) -> Result<(), String> {
    let parsed = require_https_url(url)?;
    require_stable_download_url(url)?;
    require_github_release_url(&parsed)?;
    if parsed.path().ends_with("/DropSquash.dmg") {
        return Ok(());
    }
    Err("url must point to DropSquash.dmg".to_string())
}

fn require_stable_download_url(url: &str) -> Result<(), String> {
    if url.contains('?') || url.contains('#') {
        return Err("url must not contain query or fragment".to_string());
    }
    Ok(())
}

pub(super) fn require_versioned_url(version: &str, url: &str) -> Result<(), String> {
    let Some(parsed) = crate::public_url::HttpsUrl::parse(url) else {
        return Err("url must point to the matching v<version> release".to_string());
    };
    if parsed.path().contains(&format!("/v{version}/")) {
        return Ok(());
    }
    Err("url must point to the matching v<version> release".to_string())
}

pub(super) fn require_https_url(url: &str) -> Result<crate::public_url::HttpsUrl<'_>, String> {
    require_clean("url", url)?;
    if url.contains("example.com") {
        return Err("url must not contain example.com".to_string());
    }
    crate::public_url::HttpsUrl::parse(url)
        .ok_or_else(|| "url must start with https://".to_string())
}

pub(super) fn require_homepage(url: &str) -> Result<(), String> {
    let parsed = require_https_url(url)?;
    if !parsed.host_is("github.com") {
        return Err("homepage must be the canonical DropSquash repository".to_string());
    }
    if url == "https://github.com/mt4110/drop-squash" {
        return Ok(());
    }
    Err("homepage must be the canonical DropSquash repository".to_string())
}

pub(super) fn require_sha256(value: &str) -> Result<(), String> {
    require_clean("sha256", value)?;
    if value.len() == 64
        && value
            .chars()
            .all(|char| char.is_ascii_hexdigit() && !char.is_ascii_uppercase())
        && !all_same_char(value)
    {
        return Ok(());
    }
    Err("sha256 must be a real lowercase 64-character hex checksum".to_string())
}

fn require_clean(label: &str, value: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.chars().any(char::is_whitespace) {
        return Err(format!(
            "{label} must be non-empty and contain no whitespace"
        ));
    }
    Ok(())
}

fn require_github_release_url(url: &crate::public_url::HttpsUrl<'_>) -> Result<(), String> {
    if url.host_is("github.com")
        && has_github_release_asset(url.path(), "mt4110/drop-squash/releases/download/")
    {
        return Ok(());
    }
    Err("url must point to the DropSquash GitHub Release download".to_string())
}

fn has_github_release_asset(url: &str, prefix: &str) -> bool {
    url.strip_prefix(prefix).is_some_and(|suffix| {
        let parts = suffix.split('/').collect::<Vec<_>>();
        parts.len() == 2 && parts.iter().all(|part| !part.is_empty())
    })
}

fn is_numeric_part(value: &str) -> bool {
    !value.is_empty() && value.chars().all(|char| char.is_ascii_digit())
}

fn all_same_char(value: &str) -> bool {
    value
        .chars()
        .next()
        .is_some_and(|first| value.chars().all(|char| char == first))
}
