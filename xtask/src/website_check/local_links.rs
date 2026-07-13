use std::path::Path;

use super::html_links;

pub(super) fn exists(root: &Path, path: &Path, href: &str) -> bool {
    target_path(root, path, href).is_some_and(|target| target.is_file())
}

pub(super) fn fragment_exists(root: &Path, path: &Path, href: &str) -> Result<bool, String> {
    let Some(fragment) = href_fragment(href) else {
        return Ok(true);
    };
    let Some(target) = target_path(root, path, href) else {
        return Ok(false);
    };
    let text = std::fs::read_to_string(&target).map_err(|error| error.to_string())?;
    Ok(html_links::ids(&text).iter().any(|id| id == fragment))
}

fn target_path(root: &Path, path: &Path, href: &str) -> Option<std::path::PathBuf> {
    let parent = path.parent()?;
    let target = match href_path(href) {
        "" => path.to_path_buf(),
        value => parent.join(value),
    };
    if !inside_root(root, &target) {
        return None;
    }
    if target.is_file() {
        return Some(target);
    }
    let index = target.join("index.html");
    index.is_file().then_some(index)
}

fn href_path(href: &str) -> &str {
    href.split(['#', '?']).next().unwrap_or(href)
}

fn href_fragment(href: &str) -> Option<&str> {
    href.split_once('#')
        .map(|(_, fragment)| fragment.split('?').next().unwrap_or(fragment))
        .filter(|fragment| !fragment.is_empty())
}

fn inside_root(root: &Path, target: &Path) -> bool {
    let Ok(root) = root.canonicalize() else {
        return false;
    };
    let candidate = if target.is_dir() {
        target.join("index.html")
    } else {
        target.to_path_buf()
    };
    candidate
        .canonicalize()
        .is_ok_and(|path| path.starts_with(root))
}
