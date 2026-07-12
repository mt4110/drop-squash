use crate::dmg;
use std::path::{Path, PathBuf};

const DISALLOWED_BYTES: &[u8] = b"/nix/store";
const DISALLOWED_TEXT: &str = "/nix/store";

pub fn run(paths: Vec<String>) -> Result<(), String> {
    if paths.is_empty() {
        return Err("artifact-check requires at least one file path".to_string());
    }
    for path in paths {
        read_checked(&PathBuf::from(path), "artifact")?;
    }
    println!("artifact checks passed");
    Ok(())
}

pub(crate) fn read_checked(path: &Path, label: &str) -> Result<Vec<u8>, String> {
    let bytes = dmg::read(path, label)?;
    require_canonical_name(path)?;
    if has_disallowed_reference(&bytes) {
        return Err(format!(
            "artifact contains disallowed /nix/store reference: {}",
            path.display()
        ));
    }
    Ok(bytes)
}

fn require_canonical_name(path: &Path) -> Result<(), String> {
    if path.file_name().and_then(|value| value.to_str()) == Some("DropSquash.dmg") {
        return Ok(());
    }
    Err("artifact must be named DropSquash.dmg".to_string())
}

fn has_disallowed_reference(bytes: &[u8]) -> bool {
    contains_bytes(bytes, DISALLOWED_BYTES)
        || contains_bytes(bytes, &utf16le(DISALLOWED_TEXT))
        || contains_bytes(bytes, &utf16be(DISALLOWED_TEXT))
}

fn contains_bytes(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

fn utf16le(text: &str) -> Vec<u8> {
    text.encode_utf16()
        .flat_map(|value| value.to_le_bytes())
        .collect()
}

fn utf16be(text: &str) -> Vec<u8> {
    text.encode_utf16()
        .flat_map(|value| value.to_be_bytes())
        .collect()
}

#[cfg(test)]
mod tests;
