use std::path::{Path, PathBuf};

mod external_policy;
mod href_policy;
mod html_links;
mod release_copy;
mod required_pages;
mod resource_policy;

pub fn run(args: Vec<String>) -> Result<(), String> {
    let root = PathBuf::from(args.first().map(String::as_str).unwrap_or("website"));
    let errors = check_root(&root)?;
    if errors.is_empty() {
        println!("website checks passed");
        return Ok(());
    }
    Err(errors.join("\n"))
}

pub(crate) fn check_default_root() -> Result<(), String> {
    let errors = check_root(Path::new("website"))?;
    if errors.is_empty() {
        return Ok(());
    }
    Err(errors.join("\n"))
}

fn check_root(root: &Path) -> Result<Vec<String>, String> {
    let mut errors = Vec::new();
    required_pages::check(root, &mut errors);
    release_copy::check(root, &mut errors);
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
    if href_policy::has_placeholder_url(&text) {
        errors.push(format!("{} contains placeholder URL", path.display()));
    }
    for href in html_links::hrefs(&text) {
        href_policy::check(path, &href, errors);
        external_policy::check(path, &href, errors);
        if href_policy::is_external_or_anchor(&href) {
            continue;
        }
        if !root.join(&href).is_file() {
            errors.push(format!("{} links to missing {href}", path.display()));
        }
    }
    for src in html_links::srcs(&text) {
        resource_policy::check(root, path, &src, errors);
    }
    Ok(())
}

#[cfg(test)]
mod tests;
