use std::path::Path;

const UDIF_TRAILER_SIZE: usize = 512;
const UDIF_MAGIC: &[u8] = b"koly";

pub(crate) fn read(path: &Path, label: &str) -> Result<Vec<u8>, String> {
    if !path.is_file() {
        return Err(format!("{label} target is not a file: {}", path.display()));
    }
    if path.extension().and_then(|value| value.to_str()) != Some("dmg") {
        return Err(format!("{label} target must be a DMG: {}", path.display()));
    }
    let bytes = std::fs::read(path).map_err(|error| error.to_string())?;
    if bytes.is_empty() {
        return Err(format!("{label} target is empty: {}", path.display()));
    }
    if !has_udif_trailer(&bytes) {
        return Err(format!(
            "{label} target is not a UDIF DMG: {}",
            path.display()
        ));
    }
    Ok(bytes)
}

fn has_udif_trailer(bytes: &[u8]) -> bool {
    if bytes.len() < UDIF_TRAILER_SIZE {
        return false;
    }
    let start = bytes.len() - UDIF_TRAILER_SIZE;
    &bytes[start..start + UDIF_MAGIC.len()] == UDIF_MAGIC
}
