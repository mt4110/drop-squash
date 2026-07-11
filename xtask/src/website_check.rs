use std::path::{Path, PathBuf};

pub fn run(args: Vec<String>) -> Result<(), String> {
    let root = PathBuf::from(args.first().map(String::as_str).unwrap_or("website"));
    let errors = check_root(&root)?;
    if errors.is_empty() {
        println!("website checks passed");
        return Ok(());
    }
    Err(errors.join("\n"))
}

fn check_root(root: &Path) -> Result<Vec<String>, String> {
    let mut errors = Vec::new();
    for path in html_files(root)? {
        check_html(root, &path, &mut errors)?;
    }
    Ok(errors)
}

fn html_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(root).map_err(|error| error.to_string())? {
        let path = entry.map_err(|error| error.to_string())?.path();
        if path.extension().and_then(|value| value.to_str()) == Some("html") {
            files.push(path);
        }
    }
    Ok(files)
}

fn check_html(root: &Path, path: &Path, errors: &mut Vec<String>) -> Result<(), String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    if text.contains("example.com") {
        errors.push(format!("{} contains example.com", path.display()));
    }
    for href in hrefs(&text) {
        if is_external_or_anchor(&href) {
            continue;
        }
        if !root.join(&href).is_file() {
            errors.push(format!("{} links to missing {href}", path.display()));
        }
    }
    Ok(())
}

fn hrefs(text: &str) -> Vec<String> {
    text.split("href=\"")
        .skip(1)
        .filter_map(|part| part.split('"').next())
        .map(str::to_string)
        .collect()
}

fn is_external_or_anchor(href: &str) -> bool {
    href.is_empty()
        || href.starts_with('#')
        || href.starts_with("http://")
        || href.starts_with("https://")
        || href.starts_with("mailto:")
        || href.starts_with("tel:")
}

#[cfg(test)]
mod tests;
