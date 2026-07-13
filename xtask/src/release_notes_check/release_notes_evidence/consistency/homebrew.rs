use super::value;

pub(super) fn validate(text: &str, errors: &mut Vec<String>) {
    require_artifact_url(text, errors);
    require_pr_url(text, errors);
    require_sha256(text, errors);
    require_install_sha256(text, errors);
}

fn require_artifact_url(text: &str, errors: &mut Vec<String>) {
    require_contains(
        text,
        "Artifact URL",
        "Homebrew tap PR",
        "Homebrew tap PR must include the Artifact URL",
        errors,
    );
}

fn require_pr_url(text: &str, errors: &mut Vec<String>) {
    require_contains(
        text,
        "Homebrew tap PR URL",
        "Homebrew tap PR",
        "Homebrew tap PR must include the Homebrew tap PR URL",
        errors,
    );
}

fn require_sha256(text: &str, errors: &mut Vec<String>) {
    require_contains(
        text,
        "SHA-256",
        "Homebrew tap PR",
        "Homebrew tap PR must include the lowercase SHA-256 digest",
        errors,
    );
}

fn require_install_sha256(text: &str, errors: &mut Vec<String>) {
    require_contains(
        text,
        "SHA-256",
        "Homebrew install result",
        "Homebrew install result must include the lowercase SHA-256 digest",
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
