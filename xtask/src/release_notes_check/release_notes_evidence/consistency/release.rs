use super::value;

pub(super) fn validate(text: &str, errors: &mut Vec<String>) {
    require_contains(
        text,
        "GitHub Release URL",
        "GitHub Release checksum",
        "GitHub Release checksum must include the GitHub Release URL",
        errors,
    );
    require_contains(
        text,
        "Artifact URL",
        "GitHub Release checksum",
        "GitHub Release checksum must include the Artifact URL",
        errors,
    );
    require_contains(
        text,
        "SHA-256",
        "GitHub Release checksum",
        "GitHub Release checksum must include the lowercase SHA-256 digest",
        errors,
    );
}

fn require_contains(
    text: &str,
    needle: &str,
    haystack: &str,
    message: &str,
    errors: &mut Vec<String>,
) {
    let (Some(needle), Some(haystack)) = (value::field(needle, text), value::field(haystack, text))
    else {
        return;
    };
    if haystack.contains(needle) {
        return;
    }
    errors.push(message.to_string());
}
