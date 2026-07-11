use std::path::{Path, PathBuf};

const DISALLOWED_BYTES: &[u8] = b"/nix/store";

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
    let bytes = std::fs::read(path).map_err(|error| error.to_string())?;
    if bytes.is_empty() {
        return Err(format!("artifact is empty: {}", path.display()));
    }
    if contains_bytes(&bytes, DISALLOWED_BYTES) {
        return Err(format!(
            "artifact contains disallowed /nix/store reference: {}",
            path.display()
        ));
    }
    Ok(())
}

fn contains_bytes(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

#[cfg(test)]
mod tests;
