use std::path::Path;

pub(super) fn check(root: &Path, path: &Path, src: &str, errors: &mut Vec<String>) {
    if crate::url_scheme::is_http(src) || crate::url_scheme::is_https(src) || src.starts_with("//")
    {
        errors.push(format!("{} loads external resource: {src}", path.display()));
        return;
    }
    if !src.is_empty() && !src.starts_with('#') && !local_resource_exists(root, path, src) {
        errors.push(format!("{} loads missing {src}", path.display()));
    }
}

fn local_resource_exists(root: &Path, path: &Path, src: &str) -> bool {
    let src = src_path(src);
    let Some(parent) = path.parent() else {
        return false;
    };
    let target = parent.join(src);
    let Ok(root) = root.canonicalize() else {
        return false;
    };
    target
        .canonicalize()
        .is_ok_and(|target| target.starts_with(root) && target.is_file())
}

fn src_path(src: &str) -> &str {
    src.split(['#', '?']).next().unwrap_or(src)
}
