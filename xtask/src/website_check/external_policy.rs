use std::path::Path;

pub(super) fn check(path: &Path, href: &str, errors: &mut Vec<String>) {
    if !crate::url_scheme::is_https(href) || is_allowed(href) {
        return;
    }
    errors.push(format!(
        "{} links to unapproved external URL: {href}",
        path.display()
    ));
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
