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
        || invalid_public_https_url(text)
}

fn invalid_public_https_url(text: &str) -> bool {
    text.split(|character: char| {
        character.is_whitespace() || matches!(character, '"' | '\'' | '<' | '>')
    })
    .filter(|part| starts_with_scheme(part, "https://"))
    .any(|part| crate::public_url::HttpsUrl::parse(part).is_none())
}

pub(super) fn is_external_or_anchor(href: &str) -> bool {
    href.is_empty()
        || href.starts_with('#')
        || starts_with_scheme(href, "http://")
        || starts_with_scheme(href, "https://")
        || starts_with_scheme(href, "mailto:")
        || starts_with_scheme(href, "tel:")
}

fn starts_with_scheme(value: &str, scheme: &str) -> bool {
    value
        .get(..scheme.len())
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case(scheme))
}

fn check_insecure_href(path: &Path, href: &str, errors: &mut Vec<String>) {
    if starts_with_scheme(href, "http://") {
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
