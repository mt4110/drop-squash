use std::path::Path;

pub(super) fn exists(root: &Path, path: &Path, href: &str) -> bool {
    let Some(parent) = path.parent() else {
        return false;
    };
    let target = parent.join(href_path(href));
    if !inside_root(root, &target) {
        return false;
    }
    target.is_file() || target.join("index.html").is_file()
}

fn href_path(href: &str) -> &str {
    href.split(['#', '?']).next().unwrap_or(href)
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
