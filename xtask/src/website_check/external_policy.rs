use std::path::Path;

pub(super) fn check(path: &Path, href: &str, errors: &mut Vec<String>) {
    if !href.starts_with("https://") || is_allowed(href) {
        return;
    }
    errors.push(format!(
        "{} links to unapproved external URL: {href}",
        path.display()
    ));
}

fn is_allowed(href: &str) -> bool {
    href == "https://github.com/mt4110/drop-squash"
        || href.starts_with("https://github.com/mt4110/drop-squash/issues")
}
