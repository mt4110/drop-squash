use std::io::Read;
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

fn checksum_line(path: &Path) -> Result<String, String> {
    if !path.is_file() {
        return Err(format!("checksum target is not a file: {}", path.display()));
    }
    if path.metadata().map_err(|error| error.to_string())?.len() == 0 {
        return Err(format!("checksum target is empty: {}", path.display()));
    }
    Ok(format!("{}  {}", sha256_hex(path)?, artifact_name(path)?))
}

fn artifact_name(path: &Path) -> Result<&str, String> {
    path.file_name()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("checksum target has no file name: {}", path.display()))
}

fn sha256_hex(path: &Path) -> Result<String, String> {
    let mut file = std::fs::File::open(path).map_err(|error| error.to_string())?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 16 * 1024];
    loop {
        let bytes = file.read(&mut buffer).map_err(|error| error.to_string())?;
        if bytes == 0 {
            break;
        }
        hasher.update(&buffer[..bytes]);
    }
    Ok(hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

#[cfg(test)]
mod tests;
