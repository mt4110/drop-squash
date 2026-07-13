pub(super) fn is_evidence_reference(value: &str) -> bool {
    !value.is_empty()
        && value != "TBD"
        && !super::placeholders::has_token(value)
        && (docs_ref(value)
            || public_https(value)
            || value == "Release notes"
            || labeled_public_ref(value, "GitHub Release")
            || labeled_public_ref(value, "Homebrew tap PR"))
}

fn docs_ref(value: &str) -> bool {
    let Some(path) = value
        .strip_prefix('`')
        .and_then(|value| value.strip_suffix('`'))
    else {
        return false;
    };
    path.starts_with("docs/") && !path.contains(char::is_whitespace) && !path.contains("..")
}

fn public_https(value: &str) -> bool {
    crate::public_url::HttpsUrl::parse(value).is_some_and(|url| !url.has_query_or_fragment())
}

fn labeled_public_ref(value: &str, label: &str) -> bool {
    super::reference_urls::labeled(value, label).is_some_and(public_https)
}
