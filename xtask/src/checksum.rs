use crate::dmg;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

pub fn run(paths: Vec<String>) -> Result<(), String> {
    if paths.is_empty() {
        return Err("checksum requires at least one file path".to_string());
    }
    for path in paths {
        let path = PathBuf::from(path);
        println!("{}", checksum_line(&path)?);
    }
    Ok(())
}

pub(crate) fn checksum_line(path: &Path) -> Result<String, String> {
    let bytes = dmg::read(path, "checksum")?;
    Ok(format!("{}  {}", sha256_hex(&bytes), artifact_name(path)?))
}

fn artifact_name(path: &Path) -> Result<&str, String> {
    let name = path
        .file_name()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("checksum target has no file name: {}", path.display()))?;
    if name == "DropSquash.dmg" {
        return Ok(name);
    }
    Err("checksum target must be named DropSquash.dmg".to_string())
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
mod tests;
