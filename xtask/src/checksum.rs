use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

pub fn run(paths: Vec<String>) -> Result<(), String> {
    if paths.is_empty() {
        return Err("checksum requires at least one file path".to_string());
    }
    if let Some((artifact, output)) = output_request(&paths)? {
        write_output(&artifact, &output)?;
        return Ok(());
    }
    for path in paths {
        if path == "--output" {
            return Err("checksum --output requires exactly one artifact path".to_string());
        }
        println!("{}", checksum_line(&PathBuf::from(path))?);
    }
    Ok(())
}

fn output_request(paths: &[String]) -> Result<Option<(PathBuf, PathBuf)>, String> {
    if !paths.iter().any(|value| value == "--output") {
        return Ok(None);
    }
    if paths.len() != 3 || paths[1] != "--output" {
        return Err("usage: checksum <DropSquash.dmg> --output <SHA256SUMS>".to_string());
    }
    Ok(Some((PathBuf::from(&paths[0]), PathBuf::from(&paths[2]))))
}

fn write_output(artifact: &Path, output: &Path) -> Result<(), String> {
    require_checksum_output_name(output)?;
    let line = checksum_line(artifact)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)
        .map_err(|error| format!("failed to create checksum output: {error}"))?;
    writeln!(file, "{line}").map_err(|error| format!("failed to write checksum output: {error}"))
}

fn require_checksum_output_name(path: &Path) -> Result<(), String> {
    if path.file_name().and_then(|value| value.to_str()) == Some("SHA256SUMS") {
        return Ok(());
    }
    Err("checksum output must be named SHA256SUMS".to_string())
}

pub(crate) fn checksum_line(path: &Path) -> Result<String, String> {
    let bytes = crate::artifact_check::read_checked(path, "checksum")?;
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
