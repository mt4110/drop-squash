use super::row;

pub(super) fn mismatched_verified_url_pairs(text: &str) -> Vec<&'static str> {
    let mut mismatched = Vec::new();
    require_same_origin(
        text,
        "Public website deployment",
        "Refund policy finalized",
        &mut mismatched,
    );
    mismatched
}

fn require_same_origin(
    text: &str,
    first: &'static str,
    second: &'static str,
    mismatched: &mut Vec<&'static str>,
) {
    let (Some(first_url), Some(second_url)) = (
        verified_reference(text, first),
        verified_reference(text, second),
    ) else {
        return;
    };
    if origin(first_url) == origin(second_url) {
        return;
    }
    mismatched.push(second);
}

fn verified_reference<'a>(text: &'a str, blocker: &str) -> Option<&'a str> {
    row::find(text, blocker)
        .filter(|line| row::has_status(line, blocker, "Verified"))
        .and_then(row::evidence_reference)
}

fn origin(url: &str) -> Option<&str> {
    if !crate::url_scheme::is_https(url) {
        return None;
    }
    let without_scheme = &url["https://".len()..];
    Some(without_scheme.split('/').next().unwrap_or(without_scheme))
}

#[cfg(test)]
mod tests;
