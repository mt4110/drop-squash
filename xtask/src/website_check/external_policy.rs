use std::path::Path;

pub(super) fn check(path: &Path, href: &str, errors: &mut Vec<String>) {
    if !has_https_scheme(href) || is_allowed(href) {
        return;
    }
    errors.push(format!(
        "{} links to unapproved external URL: {href}",
        path.display()
    ));
}

fn has_https_scheme(href: &str) -> bool {
    href.get(.."https://".len())
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("https://"))
}

fn is_allowed(href: &str) -> bool {
    crate::public_url::HttpsUrl::parse(href).is_some_and(|url| {
        url.host_is("github.com")
            && matches!(
                url.path().trim_end_matches('/'),
                "mt4110/drop-squash" | "mt4110/drop-squash/issues"
            )
    })
}
