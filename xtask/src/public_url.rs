use std::net::Ipv4Addr;

pub(crate) struct HttpsUrl<'a> {
    host: &'a str,
    path: &'a str,
}

impl<'a> HttpsUrl<'a> {
    pub(crate) fn parse(value: &'a str) -> Option<Self> {
        if value.chars().any(char::is_whitespace) {
            return None;
        }
        if !crate::url_scheme::is_https(value) {
            return None;
        }
        let scheme_len = "https://".len();
        let without_scheme = &value[scheme_len..];
        let (host, path) = without_scheme
            .split_once('/')
            .map(|(host, path)| (host, format_path(path)))
            .unwrap_or((without_scheme, "/"));
        (!host.is_empty() && !is_placeholder_host(host)).then_some(Self { host, path })
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

fn is_placeholder_host(host: &str) -> bool {
    let lower = host.to_ascii_lowercase();
    lower == "localhost"
        || lower.contains("...")
        || lower.contains('@')
        || lower.contains(':')
        || lower.parse::<Ipv4Addr>().is_ok()
        || lower == "example.com"
        || lower.ends_with(".example.com")
        || lower == "example.org"
        || lower.ends_with(".example.org")
        || lower == "example.net"
        || lower.ends_with(".example.net")
        || lower.ends_with(".local")
        || lower.ends_with(".test")
        || lower.ends_with(".invalid")
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
    fn accepts_uppercase_https_scheme() {
        let url = HttpsUrl::parse("HTTPS://github.com/a/b").unwrap();

        assert!(url.host_is("github.com"));
        assert_eq!(url.path(), "a/b");
    }

    #[test]
    fn rejects_whitespace_and_non_https() {
        assert!(HttpsUrl::parse("https://github.com/a b").is_none());
        assert!(HttpsUrl::parse("http://github.com/a").is_none());
    }

    #[test]
    fn rejects_placeholder_or_local_hosts() {
        assert!(HttpsUrl::parse("https://example.com/release-status").is_none());
        assert!(HttpsUrl::parse("https://download.test/DropSquash.dmg").is_none());
        assert!(HttpsUrl::parse("https://social.example.invalid/path").is_none());
        assert!(HttpsUrl::parse("https://localhost/release-status").is_none());
        assert!(HttpsUrl::parse("https://127.0.0.1/release-status").is_none());
        assert!(HttpsUrl::parse("https://127.0.0.1:8080/release-status").is_none());
        assert!(HttpsUrl::parse("https://localhost:3000/release-status").is_none());
        assert!(HttpsUrl::parse("https://example.com:443/release-status").is_none());
        assert!(HttpsUrl::parse("https://store.lemonsqueezy.com@evil.com/checkout").is_none());
        assert!(HttpsUrl::parse("https://192.168.0.10/release-status").is_none());
        assert!(HttpsUrl::parse("https://dropsquash.local/release-status").is_none());
        assert!(HttpsUrl::parse("https://.../release-status").is_none());
    }

    #[test]
    fn matches_subdomain_boundary() {
        let url = HttpsUrl::parse("https://store.lemonsqueezy.com/checkout/buy/abc").unwrap();
        let imposter = HttpsUrl::parse("https://lemonsqueezy.com.evil/checkout/buy/abc").unwrap();

        assert!(url.host_is_or_subdomain_of("lemonsqueezy.com"));
        assert!(!imposter.host_is_or_subdomain_of("lemonsqueezy.com"));
    }
}
