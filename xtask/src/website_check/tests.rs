use super::check_root;

#[test]
fn accepts_local_links() {
    let directory = tempfile::tempdir().unwrap();
    write(
        directory.path(),
        "index.html",
        r#"<a href="pricing.html">Pricing</a>"#,
    );
    write(directory.path(), "pricing.html", "<p>Pricing</p>");

    assert!(check_root(directory.path()).unwrap().is_empty());
}

#[test]
fn rejects_missing_local_links() {
    let directory = tempfile::tempdir().unwrap();
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
    write(
        directory.path(),
        "index.html",
        r##"<a href="#top">Top</a><a href="https://drop.test">External</a>"##,
    );

    assert!(check_root(directory.path()).unwrap().is_empty());
}

fn write(root: &std::path::Path, name: &str, text: &str) {
    std::fs::write(root.join(name), text).unwrap();
}
