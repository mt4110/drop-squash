use std::path::Path;

pub(super) fn check(path: &Path, href: &str, errors: &mut Vec<String>) {
    check_insecure_href(path, href, errors);
    check_disallowed_live_href(path, href, errors);
}

pub(super) fn has_placeholder_url(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    lower.contains("example.")
        || lower.contains(".example/")
        || lower.contains("localhost")
        || lower.contains(".test/")
        || lower.contains(".test")
}

pub(super) fn is_external_or_anchor(href: &str) -> bool {
    href.is_empty()
        || href.starts_with('#')
        || href.starts_with("http://")
        || href.starts_with("https://")
        || href.starts_with("mailto:")
        || href.starts_with("tel:")
}

fn check_insecure_href(path: &Path, href: &str, errors: &mut Vec<String>) {
    if href.starts_with("http://") {
        errors.push(format!("{} contains insecure link: {href}", path.display()));
    }
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
