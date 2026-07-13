use super::HttpsUrl;

#[test]
fn parses_https_host_and_path() {
    let url = HttpsUrl::parse("https://github.com/a/b?x=1").unwrap();

    assert!(url.host_is("github.com"));
    assert_eq!(url.path(), "a/b");
    assert!(url.has_query_or_fragment());
}

#[test]
fn accepts_uppercase_https_scheme() {
    let url = HttpsUrl::parse("HTTPS://github.com/a/b").unwrap();

    assert!(url.host_is("github.com"));
    assert_eq!(url.path(), "a/b");
}

#[test]
fn rejects_whitespace_and_non_https() {
    assert!(HttpsUrl::parse("https://github.com/a b").is_none());
    assert!(HttpsUrl::parse("http://github.com/a").is_none());
}

#[test]
fn rejects_placeholder_or_local_hosts() {
    assert!(HttpsUrl::parse("https://example.com/release-status").is_none());
    assert!(HttpsUrl::parse("https://download.test/DropSquash.dmg").is_none());
    assert!(HttpsUrl::parse("https://social.example.invalid/path").is_none());
    assert!(HttpsUrl::parse("https://localhost/release-status").is_none());
    assert!(HttpsUrl::parse("https://127.0.0.1/release-status").is_none());
    assert!(HttpsUrl::parse("https://127.0.0.1:8080/release-status").is_none());
    assert!(HttpsUrl::parse("https://localhost:3000/release-status").is_none());
    assert!(HttpsUrl::parse("https://example.com:443/release-status").is_none());
    assert!(HttpsUrl::parse("https://store.lemonsqueezy.com@evil.com/checkout").is_none());
    assert!(HttpsUrl::parse("https://192.168.0.10/release-status").is_none());
    assert!(HttpsUrl::parse("https://dropsquash.local/release-status").is_none());
    assert!(HttpsUrl::parse("https://.../release-status").is_none());
}

#[test]
fn matches_subdomain_boundary() {
    let url = HttpsUrl::parse("https://store.lemonsqueezy.com/checkout/buy/abc").unwrap();
    let imposter = HttpsUrl::parse("https://lemonsqueezy.com.evil/checkout/buy/abc").unwrap();

    assert!(url.host_is_or_subdomain_of("lemonsqueezy.com"));
    assert!(!imposter.host_is_or_subdomain_of("lemonsqueezy.com"));
}

#[test]
fn reports_absent_query_and_fragment() {
    let url = HttpsUrl::parse("https://github.com/a/b").unwrap();

    assert!(!url.has_query_or_fragment());
}

#[test]
fn rejects_placeholder_checkout_buy_ids() {
    assert!(super::has_checkout_buy_path("checkout/buy/abc123"));
    assert!(!super::has_checkout_buy_path("checkout/buy/example"));
    assert!(!super::has_checkout_buy_path("checkout/buy/test"));
    assert!(!super::has_checkout_buy_path("checkout/buy/abc123/extra"));
}
