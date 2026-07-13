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
    super::reference_urls::labeled(value, label).is_some_and(public_https)
}
