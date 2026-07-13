pub(crate) fn host(value: &str) -> Option<&str> {
    if !crate::url_scheme::is_https(value) {
        return None;
    }
    let without_scheme = &value["https://".len()..];
    Some(without_scheme.split('/').next().unwrap_or(without_scheme))
}

pub(crate) fn same(left: &str, right: &str) -> bool {
    host(left).is_some_and(|left| host(right).is_some_and(|right| left.eq_ignore_ascii_case(right)))
}

#[cfg(test)]
mod tests {
    use super::{host, same};

    #[test]
    fn extracts_https_host() {
        assert_eq!(
            host("HTTPS://DropSquash.app/release-status"),
            Some("DropSquash.app")
        );
    }

    #[test]
    fn compares_hosts_case_insensitively() {
        assert!(same(
            "https://DropSquash.app/release-status",
            "https://dropsquash.app/refund"
        ));
    }

    #[test]
    fn rejects_non_https_origin() {
        assert_eq!(host("http://dropsquash.app/refund"), None);
    }
}
