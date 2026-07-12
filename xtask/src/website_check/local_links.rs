use std::path::Path;

pub(super) fn exists(path: &Path, href: &str) -> bool {
    let Some(parent) = path.parent() else {
        return false;
    };
    let target = parent.join(href_path(href));
    target.is_file() || target.join("index.html").is_file()
}

fn href_path(href: &str) -> &str {
    href.split(['#', '?']).next().unwrap_or(href)
}
