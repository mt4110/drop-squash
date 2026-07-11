use std::path::Path;

pub(super) fn check(_root: &Path, path: &Path, src: &str, errors: &mut Vec<String>) {
    if src.starts_with("http://") || src.starts_with("https://") || src.starts_with("//") {
        errors.push(format!("{} loads external resource: {src}", path.display()));
        return;
    }
    if !src.is_empty() && !src.starts_with('#') && !local_resource_exists(path, src) {
        errors.push(format!("{} loads missing {src}", path.display()));
    }
}

fn local_resource_exists(path: &Path, src: &str) -> bool {
    path.parent()
        .is_some_and(|parent| parent.join(src).is_file())
}
