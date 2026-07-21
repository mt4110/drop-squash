use std::path::Path;

const FORBIDDEN_NAMES: [&str; 9] = [
    "__macosx",
    ".ds_store",
    ".private_docs",
    "_import_keys",
    ".codex",
    "authkey",
    "apikey",
    ".p8",
    ".env",
];

pub(super) fn package(root: &Path) -> Result<(), String> {
    let mut stack = vec![root.to_path_buf()];
    while let Some(path) = stack.pop() {
        let metadata = std::fs::metadata(&path).map_err(|error| error.to_string())?;
        check_name(root, &path)?;
        if metadata.is_dir() {
            for entry in std::fs::read_dir(&path).map_err(|error| error.to_string())? {
                stack.push(entry.map_err(|error| error.to_string())?.path());
            }
            continue;
        }
        check_text(root, &path)?;
    }
    Ok(())
}

fn check_name(root: &Path, path: &Path) -> Result<(), String> {
    let relative = path.strip_prefix(root).unwrap_or(path);
    let value = relative.to_string_lossy().to_ascii_lowercase();
    for forbidden in FORBIDDEN_NAMES {
        if value.contains(forbidden) {
            return Err(format!(
                "submit package contains forbidden path marker {forbidden:?}: {}",
                relative.display()
            ));
        }
    }
    Ok(())
}

fn check_text(root: &Path, path: &Path) -> Result<(), String> {
    if path.extension().is_some_and(|extension| extension == "dmg") {
        return Ok(());
    }
    let bytes = std::fs::read(path).map_err(|error| error.to_string())?;
    let text = String::from_utf8_lossy(&bytes);
    let relative = path.strip_prefix(root).unwrap_or(path);
    let errors = crate::secret_text::violations(&relative.display().to_string(), &text);
    if errors.is_empty() {
        return Ok(());
    }
    Err(errors.join("\n"))
}

#[cfg(test)]
mod tests;
