use std::path::{Path, PathBuf};

const REQUIRED_PAGES: [&str; 8] = [
    "index.html",
    "download.html",
    "pricing.html",
    "privacy.html",
    "support.html",
    "license.html",
    "refund.html",
    "changelog.html",
];

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
    check_required_pages(root, &mut errors);
    check_release_copy(root, &mut errors);
    for path in html_files(root)? {
        check_html(root, &path, &mut errors)?;
    }
    Ok(errors)
}

fn check_required_pages(root: &Path, errors: &mut Vec<String>) {
    for page in REQUIRED_PAGES {
        if !root.join(page).is_file() {
            errors.push(format!("website is missing required page: {page}"));
        }
    }
}

fn check_release_copy(root: &Path, errors: &mut Vec<String>) {
    require_page_text(root, "download.html", "DropSquash.dmg", errors);
    require_page_text(root, "download.html", "notarization", errors);
    require_page_text(root, "download.html", "checksum", errors);
    require_page_text(root, "pricing.html", "Checkout opens after", errors);
    require_page_text(
        root,
        "support.html",
        "Do not send screen recordings",
        errors,
    );
    require_page_text(root, "support.html", "app version", errors);
}

fn require_page_text(root: &Path, page: &str, needle: &str, errors: &mut Vec<String>) {
    let path = root.join(page);
    let Ok(text) = std::fs::read_to_string(&path) else {
        return;
    };
    if !text.contains(needle) {
        errors.push(format!("{page} is missing required text: {needle}"));
    }
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
        check_disallowed_live_href(path, &href, errors);
        if is_external_or_anchor(&href) {
            continue;
        }
        if !root.join(&href).is_file() {
            errors.push(format!("{} links to missing {href}", path.display()));
        }
    }
    Ok(())
}

fn check_disallowed_live_href(path: &Path, href: &str, errors: &mut Vec<String>) {
    let lower = href.to_ascii_lowercase();
    if lower.contains(".dmg") || lower.contains("lemonsqueezy") || lower.contains("checkout") {
        errors.push(format!(
            "{} contains pre-release live link: {href}",
            path.display()
        ));
    }
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
