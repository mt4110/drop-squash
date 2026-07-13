use std::path::{Path, PathBuf};

pub(super) fn check(root: &Path, errors: &mut Vec<String>) -> Result<(), String> {
    for path in collect(root)? {
        let text = std::fs::read_to_string(&path).map_err(|error| error.to_string())?;
        for src in urls(&text) {
            super::resource_policy::check(root, &path, &src, errors);
        }
    }
    Ok(())
}

fn collect(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    collect_from(root, &mut files)?;
    Ok(files)
}

fn collect_from(directory: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in std::fs::read_dir(directory).map_err(|error| error.to_string())? {
        let path = entry.map_err(|error| error.to_string())?.path();
        if path.is_dir() {
            collect_from(&path, files)?;
        } else if path.extension().and_then(|value| value.to_str()) == Some("css") {
            files.push(path);
        }
    }
    Ok(())
}

fn urls(text: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut rest = text;
    while let Some(index) = rest.to_ascii_lowercase().find("url(") {
        rest = &rest[index + "url(".len()..];
        let Some(end) = rest.find(')') else {
            break;
        };
        values.push(unquote(rest[..end].trim()).to_string());
        rest = &rest[end + 1..];
    }
    values
}

fn unquote(value: &str) -> &str {
    let quoted = value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .or_else(|| {
            value
                .strip_prefix('\'')
                .and_then(|value| value.strip_suffix('\''))
        });
    quoted.unwrap_or(value)
}
