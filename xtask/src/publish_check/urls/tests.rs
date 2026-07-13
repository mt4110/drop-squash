use super::{labeled_https, same_https, single_https};

#[test]
fn returns_one_https_url() {
    let value = "GitHub Release HTTPS://github.com/mt4110/drop-squash/releases/tag/v0.1.0";

    assert_eq!(
        single_https(value),
        Some("HTTPS://github.com/mt4110/drop-squash/releases/tag/v0.1.0")
    );
}

#[test]
fn rejects_multiple_https_urls() {
    let value = "first https://dropsquash.app/release-status second https://dropsquash.app/refund";

    assert_eq!(single_https(value), None);
}

#[test]
fn rejects_extra_words_before_labeled_url() {
    let value = "GitHub Release approved https://github.com/mt4110/drop-squash/releases/tag/v0.1.0";

    assert_eq!(labeled_https(value, "GitHub Release"), None);
}

#[test]
fn rejects_labeled_url_without_separator_space() {
    let value = "GitHub Releasehttps://github.com/mt4110/drop-squash/releases/tag/v0.1.0";

    assert_eq!(labeled_https(value, "GitHub Release"), None);
}

#[test]
fn rejects_labeled_url_with_extra_separator_space() {
    let value = "GitHub Release  https://github.com/mt4110/drop-squash/releases/tag/v0.1.0";

    assert_eq!(labeled_https(value, "GitHub Release"), None);
}

#[test]
fn compares_https_urls_with_scheme_case_ignored() {
    assert!(same_https(
        "HTTPS://github.com/mt4110/drop-squash/releases/tag/v0.1.0",
        "https://github.com/mt4110/drop-squash/releases/tag/v0.1.0"
    ));
    assert!(same_https(
        "https://GitHub.com/mt4110/drop-squash/releases/tag/v0.1.0",
        "https://github.com/mt4110/drop-squash/releases/tag/v0.1.0"
    ));
    assert!(!same_https(
        "HTTPS://github.com/mt4110/drop-squash/releases/tag/v0.2.0",
        "https://github.com/mt4110/drop-squash/releases/tag/v0.1.0"
    ));
    assert!(!same_https(
        "https://github.com/MT4110/drop-squash/releases/tag/v0.1.0",
        "https://github.com/mt4110/drop-squash/releases/tag/v0.1.0"
    ));
    assert!(!same_https(
        "https://github.com/mt4110/drop-squash/releases/tag/v0.1.0 TBD",
        "https://github.com/mt4110/drop-squash/releases/tag/v0.1.0"
    ));
    assert!(!same_https("https://", "HTTPS://"));
}
