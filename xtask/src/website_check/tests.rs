use super::check_root;

#[test]
fn accepts_local_links() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "index.html",
        r#"Release status <a href="pricing.html">Pricing</a>"#,
    );
    write(
        directory.path(),
        "pricing.html",
        required_page_text("pricing.html"),
    );

    assert!(check_root(directory.path()).unwrap().is_empty());
}

#[test]
fn accepts_local_links_with_fragments() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "index.html",
        r##"Release status <a href="pricing.html#plans">Pricing</a>"##,
    );
    write(
        directory.path(),
        "pricing.html",
        required_page_text("pricing.html"),
    );

    assert!(check_root(directory.path()).unwrap().is_empty());
}

#[test]
fn website_directory_passes_check() {
    assert!(check_root(std::path::Path::new("../website"))
        .unwrap()
        .is_empty());
}

#[test]
fn accepts_nested_release_status_page() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(directory.path(), "styles.css", "body {}");
    write(
        directory.path(),
        "index.html",
        r#"Release status <a href="release-status/">Release status</a>"#,
    );
    write(
        directory.path(),
        "release-status/index.html",
        r#"Paid beta is not public yet signed and notarized docs/release-blockers.md Evidence reference Lemon Squeezy sandbox validation <link rel="stylesheet" href="../styles.css" /><a href="../index.html">Home</a>"#,
    );

    assert!(check_root(directory.path()).unwrap().is_empty());
}

#[test]
fn accepts_local_resources_with_cache_busters() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(directory.path(), "styles.css", "body {}");
    write(
        directory.path(),
        "index.html",
        r#"Release status <script src="styles.css?v=1"></script>"#,
    );

    assert!(check_root(directory.path()).unwrap().is_empty());
}

#[test]
fn rejects_missing_local_links() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "index.html",
        r#"Release status <a href="missing.html">Missing</a>"#,
    );

    let errors = check_root(directory.path()).unwrap();

    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("missing.html"));
}

#[test]
fn rejects_example_dot_com_placeholders() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "index.html",
        r#"Release status <a href="https://example.com">Buy</a>"#,
    );

    let errors = check_root(directory.path()).unwrap();

    assert!(errors.iter().any(|error| error.contains("placeholder URL")));
}

#[test]
fn ignores_external_links_and_anchors() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "index.html",
        r##"Release status <a href="#top">Top</a><a href="https://github.com/mt4110/drop-squash">External</a>"##,
    );

    assert!(check_root(directory.path()).unwrap().is_empty());
}

#[test]
fn accepts_github_issues_link() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "index.html",
        r#"Release status <a href="https://github.com/mt4110/drop-squash/issues">Issues</a>"#,
    );

    assert!(check_root(directory.path()).unwrap().is_empty());
}

#[test]
fn rejects_allowed_github_links_with_query_or_fragment() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "index.html",
        r##"Release status <a href="https://github.com/mt4110/drop-squash?utm=1">Repo</a><a href="https://github.com/mt4110/drop-squash/issues#new">Issues</a>"##,
    );

    let errors = check_root(directory.path()).unwrap();

    assert!(errors
        .iter()
        .any(|error| error.contains("unapproved external URL")));
}

#[test]
fn rejects_unapproved_external_links() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "index.html",
        r#"Release status <a href="https://social.example.invalid/dropsquash">Social</a>"#,
    );

    let errors = check_root(directory.path()).unwrap();

    assert!(errors
        .iter()
        .any(|error| error.contains("unapproved external URL")));
}

#[test]
fn rejects_uppercase_unapproved_external_links() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "index.html",
        r#"Release status <a href="HTTPS://github.com/mt4110/drop-squash/releases">Releases</a>"#,
    );

    let errors = check_root(directory.path()).unwrap();

    assert!(errors
        .iter()
        .any(|error| error.contains("unapproved external URL")));
}

#[test]
fn rejects_github_issue_like_external_path() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "index.html",
        r#"Release status <a href="https://github.com/mt4110/drop-squash/issues-archive">Issues</a>"#,
    );

    let errors = check_root(directory.path()).unwrap();

    assert!(errors
        .iter()
        .any(|error| error.contains("unapproved external URL")));
}

#[test]
fn rejects_placeholder_external_urls() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "index.html",
        r#"Release status <a href="https://download.test/DropSquash">Download</a>"#,
    );

    let errors = check_root(directory.path()).unwrap();

    assert!(errors.iter().any(|error| error.contains("placeholder URL")));
}

#[test]
fn rejects_local_only_external_urls() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "index.html",
        r#"Release status <a href="https://192.168.0.10/release-status">Local</a><a href="https://dropsquash.local/refund">Local</a>"#,
    );

    let errors = check_root(directory.path()).unwrap();

    assert!(errors.iter().any(|error| error.contains("placeholder URL")));
}

#[test]
fn rejects_secret_like_website_values() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "pricing.html",
        "Checkout opens after signed beta release Lemon Squeezy sandbox validation No checkout link is live yet release-status/ Beta price is draft 10 successful conversions are free Failed or cancelled conversions do not count License policy product_id=123",
    );

    let errors = check_root(directory.path()).unwrap();

    assert!(errors
        .iter()
        .any(|error| error.contains("secret-like value")));
}

#[test]
fn rejects_insecure_external_links() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "support.html",
        r#"FAQ What is a privacy receipt? Does DropSquash upload my videos? Does it use ffmpeg? Do not send screen recordings app version <a href="http://dropsquash.app">Support</a>"#,
    );

    let errors = check_root(directory.path()).unwrap();

    assert!(errors.iter().any(|error| error.contains("insecure link")));
}

#[test]
fn rejects_uppercase_insecure_external_links() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "support.html",
        r#"FAQ What is a privacy receipt? Does DropSquash upload my videos? Does it use ffmpeg? Do not send screen recordings app version <a href="HTTP://dropsquash.app">Support</a>"#,
    );

    let errors = check_root(directory.path()).unwrap();

    assert!(errors.iter().any(|error| error.contains("insecure link")));
}

#[test]
fn rejects_external_loaded_resources() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "index.html",
        r#"Release status <script src="https://cdn.example.invalid/app.js"></script>"#,
    );

    let errors = check_root(directory.path()).unwrap();

    assert!(errors
        .iter()
        .any(|error| error.contains("loads external resource")));
}

#[test]
fn rejects_uppercase_external_loaded_resources() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "index.html",
        r#"Release status <script src="HTTPS://cdn.example.invalid/app.js"></script>"#,
    );

    let errors = check_root(directory.path()).unwrap();

    assert!(errors
        .iter()
        .any(|error| error.contains("loads external resource")));
}

#[test]
fn rejects_unquoted_external_loaded_resources() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "index.html",
        r#"Release status <script src=https://cdn.example.invalid/app.js></script>"#,
    );

    let errors = check_root(directory.path()).unwrap();

    assert!(errors
        .iter()
        .any(|error| error.contains("loads external resource")));
}

#[test]
fn rejects_missing_local_loaded_resources() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "index.html",
        r#"Release status <img src="missing.png" alt="" />"#,
    );

    let errors = check_root(directory.path()).unwrap();

    assert!(errors.iter().any(|error| error.contains("missing.png")));
}

#[test]
fn rejects_missing_required_pages() {
    let directory = tempfile::tempdir().unwrap();
    write(directory.path(), "index.html", "<p>Home</p>");

    let errors = check_root(directory.path()).unwrap();

    assert!(errors.iter().any(|error| error.contains("pricing.html")));
}

#[test]
fn rejects_missing_release_status_copy() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(directory.path(), "download.html", "<p>Download now</p>");

    let errors = check_root(directory.path()).unwrap();

    assert!(errors.iter().any(|error| error.contains("DropSquash.dmg")));
    assert!(errors.iter().any(|error| error.contains("notarization")));
    assert!(errors.iter().any(|error| error.contains("checksum")));
}

#[test]
fn rejects_missing_privacy_copy() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "privacy.html",
        "<p>Private by design.</p>",
    );

    let errors = check_root(directory.path()).unwrap();

    assert!(errors.iter().any(|error| error.contains("upload media")));
    assert!(errors.iter().any(|error| error.contains("Telemetry")));
    assert!(errors
        .iter()
        .any(|error| error.contains("privacy receipts")));
    assert!(errors
        .iter()
        .any(|error| error.contains("metadata_policy = preserve")));
    assert!(errors
        .iter()
        .any(|error| error.contains("uploaded_bytes = 0")));
    assert!(errors
        .iter()
        .any(|error| error.contains("metadata_policy = preserve")));
    assert!(errors.iter().any(|error| error.contains("Lemon Squeezy")));
}

#[test]
fn rejects_missing_license_copy() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(directory.path(), "license.html", "<p>License active.</p>");

    let errors = check_root(directory.path()).unwrap();

    assert!(errors
        .iter()
        .any(|error| error.contains("license-key fingerprint")));
    assert!(errors.iter().any(|error| error.contains("raw license key")));
    assert!(errors
        .iter()
        .any(|error| error.contains("local license cache")));
    assert!(errors.iter().any(|error| error.contains("Offline grace")));
}

#[test]
fn rejects_missing_refund_copy() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(directory.path(), "refund.html", "<p>Refunds available.</p>");

    let errors = check_root(directory.path()).unwrap();

    assert!(errors.iter().any(|error| error.contains("draft policy")));
    assert!(errors.iter().any(|error| error.contains("cannot activate")));
    assert!(errors.iter().any(|error| error.contains("order email")));
}

#[test]
fn rejects_missing_pricing_draft_copy() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(directory.path(), "pricing.html", "<p>$29</p>");

    let errors = check_root(directory.path()).unwrap();

    assert!(errors
        .iter()
        .any(|error| error.contains("Beta price is draft")));
}

#[test]
fn rejects_missing_pricing_refund_link() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "pricing.html",
        "Checkout opens after signed beta release Lemon Squeezy sandbox validation No checkout link is live yet release-status/ Beta price is draft 10 successful conversions are free Failed or cancelled conversions do not count License policy",
    );

    let errors = check_root(directory.path()).unwrap();

    assert!(errors.iter().any(|error| error.contains("refund.html")));
}

#[test]
fn rejects_pre_release_cta_copy() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "download.html",
        "macOS beta DropSquash.dmg notarization checksum release-status/ Download now",
    );
    write(directory.path(), "pricing.html", "Checkout opens after signed beta release Lemon Squeezy sandbox validation No checkout link is live yet release-status/ Beta price is draft 10 successful conversions are free Failed or cancelled conversions do not count License policy Buy now");

    let errors = check_root(directory.path()).unwrap();

    assert!(errors.iter().any(|error| error.contains("download now")));
    assert!(errors.iter().any(|error| error.contains("buy now")));
}

#[test]
fn rejects_pre_release_beta_and_checkout_cta_copy() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "download.html",
        "macOS beta DropSquash.dmg notarization checksum release-status/ Download the beta",
    );
    write(directory.path(), "pricing.html", "Checkout opens after signed beta release Lemon Squeezy sandbox validation No checkout link is live yet release-status/ Beta price is draft 10 successful conversions are free Failed or cancelled conversions do not count License policy Start checkout");

    let errors = check_root(directory.path()).unwrap();

    assert!(errors
        .iter()
        .any(|error| error.contains("download the beta")));
    assert!(errors.iter().any(|error| error.contains("start checkout")));
}

#[test]
fn rejects_pre_release_product_and_platform_cta_copy() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "download.html",
        "macOS beta DropSquash.dmg notarization checksum release-status/ Download DropSquash. Download for macOS.",
    );
    write(directory.path(), "pricing.html", "Checkout opens after signed beta release Lemon Squeezy sandbox validation No checkout link is live yet release-status/ Beta price is draft 10 successful conversions are free Failed or cancelled conversions do not count License policy refund.html Buy DropSquash");

    let errors = check_root(directory.path()).unwrap();

    assert!(errors
        .iter()
        .any(|error| error.contains("download dropsquash")));
    assert!(errors
        .iter()
        .any(|error| error.contains("download for macos")));
    assert!(errors.iter().any(|error| error.contains("buy dropsquash")));
}

#[test]
fn rejects_pre_release_download_or_checkout_links() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "download.html",
        r#"<a href="https://downloads.test/DropSquash.dmg">Download</a>"#,
    );
    write(
        directory.path(),
        "pricing.html",
        r#"<a href="https://store.lemonsqueezy.com/checkout/test">Buy</a>"#,
    );

    let errors = check_root(directory.path()).unwrap();

    assert!(errors.iter().any(|error| error.contains("DropSquash.dmg")));
    assert!(errors.iter().any(|error| error.contains("lemonsqueezy")));
}

#[test]
fn rejects_pre_release_alternate_artifact_links() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "download.html",
        r#"<a href="https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.zip">Download</a><a href="https://example.invalid/DropSquash.pkg">PKG</a>"#,
    );

    let errors = check_root(directory.path()).unwrap();

    assert!(errors.iter().any(|error| error.contains(".zip")));
    assert!(errors.iter().any(|error| error.contains(".pkg")));
}

#[test]
fn rejects_pre_release_checkout_form_actions() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "pricing.html",
        r#"<form action="https://store.lemonsqueezy.com/checkout/buy/abc123"></form>"#,
    );

    let errors = check_root(directory.path()).unwrap();

    assert!(errors.iter().any(|error| error.contains("lemonsqueezy")));
}

#[test]
fn rejects_unquoted_pre_release_checkout_form_actions() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "pricing.html",
        r#"<form action=https://store.lemonsqueezy.com/checkout/buy/abc123></form>"#,
    );

    let errors = check_root(directory.path()).unwrap();

    assert!(errors.iter().any(|error| error.contains("lemonsqueezy")));
}

#[test]
fn rejects_pre_release_links_with_single_quotes_or_uppercase_href() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "download.html",
        r#"<a HREF='https://downloads.test/DropSquash.dmg'>Download</a>"#,
    );

    let errors = check_root(directory.path()).unwrap();

    assert!(errors.iter().any(|error| error.contains("DropSquash.dmg")));
}

#[test]
fn rejects_pre_release_unquoted_checkout_links() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "pricing.html",
        r#"<a href=https://store.lemonsqueezy.com/checkout/buy/abc123>Buy</a>"#,
    );

    let errors = check_root(directory.path()).unwrap();

    assert!(errors.iter().any(|error| error.contains("lemonsqueezy")));
}

#[test]
fn rejects_unsupported_platform_availability_claims() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "download.html",
        "macOS beta DropSquash.dmg notarization checksum Download for Windows",
    );

    let errors = check_root(directory.path()).unwrap();

    assert!(errors
        .iter()
        .any(|error| error.contains("unsupported platform")));
}

#[test]
fn rejects_lowercase_unsupported_platform_availability_claims() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "download.html",
        "macOS beta DropSquash.dmg notarization checksum available on windows",
    );

    let errors = check_root(directory.path()).unwrap();

    assert!(errors
        .iter()
        .any(|error| error.contains("unsupported platform")));
}

fn write_required_pages(root: &std::path::Path) {
    for page in [
        "index.html",
        "release-status/index.html",
        "download.html",
        "pricing.html",
        "privacy.html",
        "support.html",
        "license.html",
        "refund.html",
        "changelog.html",
    ] {
        write(root, page, required_page_text(page));
    }
}

fn required_page_text(page: &str) -> &'static str {
    match page {
        "index.html" => "Release status",
        "release-status/index.html" => {
            "Paid beta is not public yet signed and notarized docs/release-blockers.md Evidence reference Lemon Squeezy sandbox validation"
        }
        "download.html" => "macOS beta DropSquash.dmg notarization checksum release-status/",
        "pricing.html" => {
            "Checkout opens after signed beta release Lemon Squeezy sandbox validation No checkout link is live yet release-status/ Beta price is draft 10 successful conversions are free refund.html Failed or cancelled conversions do not count License policy"
        }
        "privacy.html" => {
            "does not upload media Telemetry is off by default privacy receipts uploaded_bytes = 0 metadata_policy = preserve file names instead of absolute paths License activation contacts Lemon Squeezy"
        }
        "support.html" => {
            "FAQ What is a privacy receipt? metadata_policy = preserve Does DropSquash upload my videos? does not upload media Does it use ffmpeg? does not shell out ffprobe Do not send screen recordings app version GitHub Issues paid beta support address"
        }
        "license.html" => {
            "license-key fingerprint does not persist the raw license key local license cache Offline grace Server-side deactivation not automatic"
        }
        "refund.html" => {
            "draft policy checkout goes live cannot activate basic local conversion workflow order email Lemon Squeezy order flow"
        }
        _ => "<p>Page</p>",
    }
}

fn write(root: &std::path::Path, name: &str, text: &str) {
    let path = root.join(name);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}
