pub(crate) struct HttpsUrl<'a> {
    host: &'a str,
    path: &'a str,
}

impl<'a> HttpsUrl<'a> {
    pub(crate) fn parse(value: &'a str) -> Option<Self> {
        if value.chars().any(char::is_whitespace) {
            return None;
        }
        let without_scheme = value.strip_prefix("https://")?;
        let (host, path) = without_scheme
            .split_once('/')
            .map(|(host, path)| (host, format_path(path)))
            .unwrap_or((without_scheme, "/"));
        (!host.is_empty()).then_some(Self { host, path })
    }

    pub(crate) fn host_is(&self, expected: &str) -> bool {
        self.host.eq_ignore_ascii_case(expected)
    }

    pub(crate) fn host_is_or_subdomain_of(&self, expected: &str) -> bool {
        self.host_is(expected)
            || self
                .host
                .to_ascii_lowercase()
                .ends_with(&format!(".{expected}"))
    }

    pub(crate) fn path(&self) -> &str {
        self.path
    }
}

fn format_path(path: &str) -> &str {
    path.split(['?', '#']).next().unwrap_or(path)
}

#[cfg(test)]
mod tests {
    use super::HttpsUrl;

    #[test]
    fn parses_https_host_and_path() {
        let url = HttpsUrl::parse("https://github.com/a/b?x=1").unwrap();

        assert!(url.host_is("github.com"));
        assert_eq!(url.path(), "a/b");
    }

    #[test]
    fn rejects_whitespace_and_non_https() {
        assert!(HttpsUrl::parse("https://github.com/a b").is_none());
        assert!(HttpsUrl::parse("http://github.com/a").is_none());
    }

    #[test]
    fn matches_subdomain_boundary() {
        let url = HttpsUrl::parse("https://store.lemonsqueezy.com/checkout/buy/abc").unwrap();
        let imposter = HttpsUrl::parse("https://lemonsqueezy.com.evil/checkout/buy/abc").unwrap();

        assert!(url.host_is_or_subdomain_of("lemonsqueezy.com"));
        assert!(!imposter.host_is_or_subdomain_of("lemonsqueezy.com"));
    }
}
