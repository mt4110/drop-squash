use super::check_root;

#[test]
fn accepts_local_links() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "index.html",
        r#"<a href="pricing.html">Pricing</a>"#,
    );
    write(directory.path(), "pricing.html", "Checkout opens after");

    assert!(check_root(directory.path()).unwrap().is_empty());
}

#[test]
fn rejects_missing_local_links() {
    let directory = tempfile::tempdir().unwrap();
    write_required_pages(directory.path());
    write(
        directory.path(),
        "index.html",
        r#"<a href="missing.html">Missing</a>"#,
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
        r#"<a href="https://example.com">Buy</a>"#,
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
        r##"<a href="#top">Top</a><a href="https://drop.test">External</a>"##,
    );

    assert!(check_root(directory.path()).unwrap().is_empty());
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
        "download.html" => "DropSquash.dmg notarization checksum",
        "pricing.html" => "Checkout opens after",
        _ => "<p>Page</p>",
    }
}

fn write(root: &std::path::Path, name: &str, text: &str) {
    std::fs::write(root.join(name), text).unwrap();
}
