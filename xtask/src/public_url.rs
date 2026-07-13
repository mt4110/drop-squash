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

    pub(crate) fn path(&self) -> &'a str {
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
mod tests;
