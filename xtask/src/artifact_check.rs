use std::path::{Path, PathBuf};

const DISALLOWED_BYTES: &[u8] = b"/nix/store";
const UDIF_TRAILER_SIZE: usize = 512;
const UDIF_MAGIC: &[u8] = b"koly";

pub fn run(paths: Vec<String>) -> Result<(), String> {
    if paths.is_empty() {
        return Err("artifact-check requires at least one file path".to_string());
    }
    for path in paths {
        check_file(&PathBuf::from(path))?;
    }
    println!("artifact checks passed");
    Ok(())
}

fn check_file(path: &Path) -> Result<(), String> {
    if !path.is_file() {
        return Err(format!("artifact target is not a file: {}", path.display()));
    }
    if path.extension().and_then(|value| value.to_str()) != Some("dmg") {
        return Err(format!("artifact target must be a DMG: {}", path.display()));
    }
    let bytes = std::fs::read(path).map_err(|error| error.to_string())?;
    if bytes.is_empty() {
        return Err(format!("artifact is empty: {}", path.display()));
    }
    if !has_udif_trailer(&bytes) {
        return Err(format!("artifact is not a UDIF DMG: {}", path.display()));
    }
    if contains_bytes(&bytes, DISALLOWED_BYTES) {
        return Err(format!(
            "artifact contains disallowed /nix/store reference: {}",
            path.display()
        ));
    }
    Ok(())
}

fn has_udif_trailer(bytes: &[u8]) -> bool {
    if bytes.len() < UDIF_TRAILER_SIZE {
        return false;
    }
    let start = bytes.len() - UDIF_TRAILER_SIZE;
    &bytes[start..start + UDIF_MAGIC.len()] == UDIF_MAGIC
}

fn contains_bytes(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

#[cfg(test)]
mod tests;
