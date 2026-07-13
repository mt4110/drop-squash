pub(super) fn single(value: &str) -> Option<&str> {
    let mut urls = value
        .split_whitespace()
        .filter(|part| crate::url_scheme::is_https(part));
    let first = urls.next()?;
    urls.next().is_none().then_some(first)
}
