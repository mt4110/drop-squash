pub(super) fn single_https(value: &str) -> Option<&str> {
    let mut urls = value
        .split_whitespace()
        .filter(|part| crate::url_scheme::is_https(part));
    let first = urls.next()?;
    urls.next().is_none().then_some(first)
}

pub(super) fn labeled_https<'a>(value: &'a str, label: &str) -> Option<&'a str> {
    let rest = value.strip_prefix(label)?.trim();
    crate::url_scheme::is_https(rest).then_some(rest)
}

pub(super) fn same_https(left: &str, right: &str) -> bool {
    without_scheme(left).is_some_and(|left| without_scheme(right) == Some(left))
}

fn without_scheme(value: &str) -> Option<&str> {
    crate::url_scheme::is_https(value).then_some(&value["https://".len()..])
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
    fn compares_https_urls_with_scheme_case_ignored() {
        assert!(same_https(
            "HTTPS://github.com/mt4110/drop-squash/releases/tag/v0.1.0",
            "https://github.com/mt4110/drop-squash/releases/tag/v0.1.0"
        ));
        assert!(!same_https(
            "HTTPS://github.com/mt4110/drop-squash/releases/tag/v0.2.0",
            "https://github.com/mt4110/drop-squash/releases/tag/v0.1.0"
        ));
    }
}
