pub(super) fn single_https(value: &str) -> Option<&str> {
    let mut urls = value
        .split_whitespace()
        .filter(|part| crate::url_scheme::is_https(part));
    let first = urls.next()?;
    urls.next().is_none().then_some(first)
}

pub(super) fn labeled_https<'a>(value: &'a str, label: &str) -> Option<&'a str> {
    let rest = value.strip_prefix(label)?.strip_prefix(' ')?;
    crate::url_scheme::is_https(rest).then_some(rest)
}

pub(super) fn same_https(left: &str, right: &str) -> bool {
    parts(left).is_some_and(|left| {
        parts(right).is_some_and(|right| {
            left.host.eq_ignore_ascii_case(right.host) && left.rest == right.rest
        })
    })
}

struct Parts<'a> {
    host: &'a str,
    rest: &'a str,
}

fn parts(value: &str) -> Option<Parts<'_>> {
    if !crate::url_scheme::is_https(value) {
        return None;
    }
    let without_scheme = &value["https://".len()..];
    let (host, rest) = without_scheme
        .split_once('/')
        .unwrap_or((without_scheme, ""));
    Some(Parts { host, rest })
}

#[cfg(test)]
mod tests {
    use super::{labeled_https, same_https, single_https};

    #[test]
    fn returns_one_https_url() {
        let value = "GitHub Release HTTPS://github.com/mt4110/drop-squash/releases/tag/v0.1.0";

        assert_eq!(
            single_https(value),
            Some("HTTPS://github.com/mt4110/drop-squash/releases/tag/v0.1.0")
        );
    }

    #[test]
    fn rejects_multiple_https_urls() {
        let value =
            "first https://dropsquash.app/release-status second https://dropsquash.app/refund";

        assert_eq!(single_https(value), None);
    }

    #[test]
    fn rejects_extra_words_before_labeled_url() {
        let value =
            "GitHub Release approved https://github.com/mt4110/drop-squash/releases/tag/v0.1.0";

        assert_eq!(labeled_https(value, "GitHub Release"), None);
    }

    #[test]
    fn rejects_labeled_url_without_separator_space() {
        let value = "GitHub Releasehttps://github.com/mt4110/drop-squash/releases/tag/v0.1.0";

        assert_eq!(labeled_https(value, "GitHub Release"), None);
    }

    #[test]
    fn rejects_labeled_url_with_extra_separator_space() {
        let value = "GitHub Release  https://github.com/mt4110/drop-squash/releases/tag/v0.1.0";

        assert_eq!(labeled_https(value, "GitHub Release"), None);
    }

    #[test]
    fn compares_https_urls_with_scheme_case_ignored() {
        assert!(same_https(
            "HTTPS://github.com/mt4110/drop-squash/releases/tag/v0.1.0",
            "https://github.com/mt4110/drop-squash/releases/tag/v0.1.0"
        ));
        assert!(same_https(
            "https://GitHub.com/mt4110/drop-squash/releases/tag/v0.1.0",
            "https://github.com/mt4110/drop-squash/releases/tag/v0.1.0"
        ));
        assert!(!same_https(
            "HTTPS://github.com/mt4110/drop-squash/releases/tag/v0.2.0",
            "https://github.com/mt4110/drop-squash/releases/tag/v0.1.0"
        ));
        assert!(!same_https(
            "https://github.com/MT4110/drop-squash/releases/tag/v0.1.0",
            "https://github.com/mt4110/drop-squash/releases/tag/v0.1.0"
        ));
    }
}
