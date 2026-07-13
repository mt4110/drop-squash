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
    .filter(|part| crate::url_scheme::is_https(part))
    .any(|part| crate::public_url::HttpsUrl::parse(part).is_none())
}

pub(super) fn is_external_or_anchor(href: &str) -> bool {
    href.is_empty()
        || href.starts_with('#')
        || crate::url_scheme::is_http(href)
        || crate::url_scheme::is_https(href)
        || href
            .get(.."mailto:".len())
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case("mailto:"))
        || href
            .get(.."tel:".len())
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case("tel:"))
}

fn check_insecure_href(path: &Path, href: &str, errors: &mut Vec<String>) {
    if crate::url_scheme::is_http(href) {
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
