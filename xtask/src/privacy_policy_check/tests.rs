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
fn rejects_network_clients_outside_license_provider() {
    let directory = tempfile::tempdir().unwrap();
    write(
        directory.path(),
        "crates/dropsquash-encoder/src/lib.rs",
        "reqwest::Client::new()",
    );

    let error = check_roots(&[directory.path().join("crates")]).unwrap_err();

    assert!(error.contains("network marker reqwest"));
}

#[test]
fn rejects_alternate_rust_network_clients_outside_license_provider() {
    let directory = tempfile::tempdir().unwrap();
    write(
        directory.path(),
        "crates/dropsquash-core/src/upload.rs",
        "ureq::get(\"https://example.com\"); hyper::Client::new(); tokio_tungstenite::connect_async(url);",
    );

    let error = check_roots(&[directory.path().join("crates")]).unwrap_err();

    assert!(error.contains("network marker ureq::"));
    assert!(error.contains("network marker hyper::"));
    assert!(error.contains("network marker tokio_tungstenite"));
}

#[test]
fn allows_license_provider_network_client() {
    let directory = tempfile::tempdir().unwrap();
    write(
        directory.path(),
        "crates/dropsquash-license/src/lemonsqueezy/transport.rs",
        "reqwest::Client::new()",
    );

    assert!(check_roots(&[directory.path().join("crates")]).is_ok());
}

#[test]
fn rejects_license_network_client_outside_transport() {
    let directory = tempfile::tempdir().unwrap();
    write(
        directory.path(),
        "crates/dropsquash-license/src/cache.rs",
        "reqwest::Client::new()",
    );

    let error = check_roots(&[directory.path().join("crates")]).unwrap_err();

    assert!(error.contains("network marker reqwest"));
}

#[test]
fn rejects_frontend_fetch_calls() {
    let directory = tempfile::tempdir().unwrap();
    write(
        directory.path(),
        "apps/desktop/web/src/App.tsx",
        "fetch('/upload')",
    );

    let error = check_roots(&[directory.path().join("apps")]).unwrap_err();

    assert!(error.contains("network marker fetch("));
}

#[test]
fn rejects_frontend_http_client_packages() {
    let directory = tempfile::tempdir().unwrap();
    write(
        directory.path(),
        "apps/desktop/web/src/api.ts",
        "import axios from 'axios';",
    );
    write(
        directory.path(),
        "apps/desktop/web/src/plugin.ts",
        "import { fetch } from '@tauri-apps/plugin-http';",
    );

    let error = check_roots(&[directory.path().join("apps")]).unwrap_err();

    assert!(error.contains("network marker axios"));
    assert!(error.contains("network marker @tauri-apps/plugin-http"));
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
