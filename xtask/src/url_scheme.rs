pub(crate) fn is_http(value: &str) -> bool {
    starts_with(value, "http://")
}

pub(crate) fn is_https(value: &str) -> bool {
    starts_with(value, "https://")
}

fn starts_with(value: &str, scheme: &str) -> bool {
    value
        .get(..scheme.len())
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case(scheme))
}

#[cfg(test)]
mod tests {
    use super::{is_http, is_https};

    #[test]
    fn matches_scheme_case_insensitively() {
        assert!(is_http("HTTP://dropsquash.app"));
        assert!(is_https("HTTPS://dropsquash.app"));
    }

    #[test]
    fn rejects_missing_scheme() {
        assert!(!is_http("dropsquash.app"));
        assert!(!is_https("//dropsquash.app"));
    }
}
