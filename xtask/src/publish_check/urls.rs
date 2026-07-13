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
    if value.chars().any(char::is_whitespace) {
        return None;
    }
    if !crate::url_scheme::is_https(value) {
        return None;
    }
    let without_scheme = &value["https://".len()..];
    let (host, rest) = without_scheme
        .split_once('/')
        .unwrap_or((without_scheme, ""));
    if host.is_empty() {
        return None;
    }
    Some(Parts { host, rest })
}

#[cfg(test)]
mod tests;
