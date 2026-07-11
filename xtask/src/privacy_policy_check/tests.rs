use super::check_roots;

#[test]
fn accepts_clean_product_sources() {
    let directory = tempfile::tempdir().unwrap();
    write(
        directory.path(),
        "apps/desktop/web/src/App.tsx",
        "export {};",
    );

    assert!(check_roots(&[directory.path().join("apps")]).is_ok());
}

#[test]
fn rejects_analytics_sdk_names() {
    let directory = tempfile::tempdir().unwrap();
    write(
        directory.path(),
        "apps/desktop/web/src/App.tsx",
        "posthog.init()",
    );

    let error = check_roots(&[directory.path().join("apps")]).unwrap_err();

    assert!(error.contains("posthog"));
}

#[test]
fn rejects_browser_beacon_api() {
    let directory = tempfile::tempdir().unwrap();
    write(
        directory.path(),
        "apps/desktop/web/src/App.tsx",
        "navigator.sendBeacon('/event')",
    );

    let error = check_roots(&[directory.path().join("apps")]).unwrap_err();

    assert!(error.contains("sendbeacon"));
}

#[test]
fn skips_built_dist_output() {
    let directory = tempfile::tempdir().unwrap();
    write(
        directory.path(),
        "apps/desktop/web/dist/index.js",
        "gtag('event')",
    );

    assert!(check_roots(&[directory.path().join("apps")]).is_ok());
}

fn write(root: &std::path::Path, name: &str, text: &str) {
    let path = root.join(name);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}
