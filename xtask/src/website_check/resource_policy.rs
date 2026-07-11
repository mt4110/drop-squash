use std::path::Path;

pub(super) fn check(root: &Path, path: &Path, src: &str, errors: &mut Vec<String>) {
    if src.starts_with("http://") || src.starts_with("https://") || src.starts_with("//") {
        errors.push(format!("{} loads external resource: {src}", path.display()));
        return;
    }
    if !src.is_empty() && !src.starts_with('#') && !root.join(src).is_file() {
        errors.push(format!("{} loads missing {src}", path.display()));
    }
}
