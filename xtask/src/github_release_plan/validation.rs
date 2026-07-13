use std::path::Path;

pub(super) fn tag(tag: &str) -> Result<(), String> {
    let Some(version) = tag.strip_prefix('v') else {
        return Err("GitHub Release tag must look like v1.2.3".to_string());
    };
    let parts = version.split('.').collect::<Vec<_>>();
    if parts.len() == 3 && parts.iter().all(|part| is_numeric_part(part)) {
        return Ok(());
    }
    Err("GitHub Release tag must look like v1.2.3".to_string())
}

pub(super) fn checksum(path: &Path) -> Result<String, String> {
    if path.file_name().and_then(|value| value.to_str()) != Some("SHA256SUMS") {
        return Err("GitHub Release checksum must be named SHA256SUMS".to_string());
    }
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read SHA256SUMS: {error}"))?;
    if text.contains("/nix/store") {
        return Err("SHA256SUMS must not contain /nix/store".to_string());
    }
    let line = text
        .lines()
        .find(|line| line.ends_with("  DropSquash.dmg"))
        .ok_or_else(|| "SHA256SUMS must contain the DropSquash.dmg checksum line".to_string())?;
    let Some((digest, _)) = line.split_once("  ") else {
        return Err("SHA256SUMS must contain the DropSquash.dmg checksum line".to_string());
    };
    if is_sha256(digest) {
        return Ok(digest.to_string());
    }
    Err("SHA256SUMS DropSquash.dmg digest must be lowercase SHA-256".to_string())
}

pub(super) fn notes(path: &Path, tag: &str, digest: &str) -> Result<(), String> {
    if path.extension().and_then(|value| value.to_str()) != Some("md") || !path.is_file() {
        return Err("GitHub Release notes must be an existing .md file".to_string());
    }
    crate::release_notes_check::check_file_silent(path)
        .map_err(|error| format!("GitHub Release notes must pass release-notes-check:\n{error}"))?;
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read GitHub Release notes: {error}"))?;
    require_line(&text, &format!("- Version: {tag}"), "tag")?;
    require_line(&text, &format!("- SHA-256: {digest}"), "SHA256SUMS digest")?;
    require_contains(&text, &download_url(tag), "Artifact URL")
}

fn require_line(text: &str, expected: &str, label: &str) -> Result<(), String> {
    if text.lines().any(|line| line.trim() == expected) {
        return Ok(());
    }
    Err(format!("GitHub Release notes must match the {label}"))
}

fn require_contains(text: &str, expected: &str, label: &str) -> Result<(), String> {
    if text.contains(expected) {
        return Ok(());
    }
    Err(format!("GitHub Release notes must match the {label}"))
}

fn download_url(tag: &str) -> String {
    format!("https://github.com/mt4110/drop-squash/releases/download/{tag}/DropSquash.dmg")
}

fn is_numeric_part(part: &str) -> bool {
    !part.is_empty() && part.chars().all(|ch| ch.is_ascii_digit())
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .chars()
            .all(|ch| ch.is_ascii_hexdigit() && !ch.is_ascii_uppercase())
}
