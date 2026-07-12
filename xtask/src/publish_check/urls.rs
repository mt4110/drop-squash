pub(super) fn single_https(value: &str) -> Option<&str> {
    let mut urls = value
        .split_whitespace()
        .filter(|part| part.starts_with("https://"));
    let first = urls.next()?;
    urls.next().is_none().then_some(first)
}

#[cfg(test)]
mod tests {
    use super::single_https;

    #[test]
    fn returns_one_https_url() {
        let value = "GitHub Release https://github.com/mt4110/drop-squash/releases/tag/v0.1.0";

        assert_eq!(
            single_https(value),
            Some("https://github.com/mt4110/drop-squash/releases/tag/v0.1.0")
        );
    }

    #[test]
    fn rejects_multiple_https_urls() {
        let value =
            "first https://dropsquash.app/release-status second https://dropsquash.app/refund";

        assert_eq!(single_https(value), None);
    }
}
