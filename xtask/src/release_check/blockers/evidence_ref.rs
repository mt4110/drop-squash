pub(super) fn is_evidence_reference(value: &str) -> bool {
    !value.is_empty()
        && value != "TBD"
        && !super::placeholders::has_token(value)
        && (value.starts_with("`docs/")
            || public_https(value)
            || value == "Release notes"
            || labeled_public_ref(value, "GitHub Release")
            || labeled_public_ref(value, "Homebrew tap PR"))
}

fn public_https(value: &str) -> bool {
    crate::public_url::HttpsUrl::parse(value).is_some()
}

fn labeled_public_ref(value: &str, label: &str) -> bool {
    let Some(rest) = value.strip_prefix(label) else {
        return false;
    };
    let mut urls = rest
        .split_whitespace()
        .filter(|part| part.starts_with("https://"));
    let Some(url) = urls.next() else {
        return false;
    };
    urls.next().is_none() && public_https(url)
}
