pub(super) fn labeled<'a>(value: &'a str, label: &str) -> Option<&'a str> {
    let rest = value.strip_prefix(label)?.strip_prefix(' ')?;
    crate::url_scheme::is_https(rest).then_some(rest)
}
