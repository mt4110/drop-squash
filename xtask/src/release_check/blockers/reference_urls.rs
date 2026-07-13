pub(super) fn labeled<'a>(value: &'a str, label: &str) -> Option<&'a str> {
    let rest = value.strip_prefix(label)?.trim();
    crate::url_scheme::is_https(rest).then_some(rest)
}
