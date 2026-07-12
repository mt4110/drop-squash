use std::path::{Path, PathBuf};

pub(super) fn collect(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    collect_from(root, &mut files)?;
    Ok(files)
}

fn collect_from(directory: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in std::fs::read_dir(directory).map_err(|error| error.to_string())? {
        let path = entry.map_err(|error| error.to_string())?.path();
        if path.is_dir() {
            collect_from(&path, files)?;
        } else if path.extension().and_then(|value| value.to_str()) == Some("html") {
            files.push(path);
        }
    }
    Ok(())
}
