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

    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("example.com"));
}

#[test]
fn ignores_external_links_and_anchors() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "index.html",
        r##"Release status <a href="#top">Top</a><a href="https://drop.test">External</a>"##,
    );

    assert!(check_root(directory.path()).unwrap().is_empty());
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
        .any(|error| error.contains("uploaded_bytes = 0")));
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

fn write_required_pages(root: &std::path::Path) {
    for page in [
        "index.html",
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
        "download.html" => "macOS beta DropSquash.dmg notarization checksum",
        "pricing.html" => {
            "Checkout opens after signed beta release 10 successful conversions are free Failed or cancelled conversions do not count"
        }
        "privacy.html" => {
            "does not upload media Telemetry is off by default privacy receipts uploaded_bytes = 0 file names instead of absolute paths License activation contacts Lemon Squeezy"
        }
        "support.html" => {
            "FAQ What is a privacy receipt? Does DropSquash upload my videos? Does it use ffmpeg? Do not send screen recordings app version GitHub Issues paid beta support address"
        }
        "license.html" => {
            "license-key fingerprint does not persist the raw license key local license cache Offline grace"
        }
        "refund.html" => {
            "draft policy checkout goes live cannot activate basic local conversion workflow order email Lemon Squeezy order flow"
        }
        _ => "<p>Page</p>",
    }
}

fn write(root: &std::path::Path, name: &str, text: &str) {
    std::fs::write(root.join(name), text).unwrap();
}
