use super::check_default_capability;

#[test]
fn accepts_minimal_desktop_permissions() {
    let (_directory, path) = capability(
        r#"[
  "core:event:default",
  "dialog:allow-open",
  "opener:allow-reveal-item-in-dir"
]"#,
    );

    assert!(check_default_capability(&path).is_ok());
}

#[test]
fn rejects_url_open_permission() {
    let (_directory, path) = capability(
        r#"[
  "core:event:default",
  "dialog:allow-open",
  "opener:allow-reveal-item-in-dir",
  "opener:allow-open-url"
]"#,
    );

    let error = check_default_capability(&path).unwrap_err();

    assert!(error.contains("disallowed permission"));
}

#[test]
fn rejects_missing_permissions_array() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("default.json");
    std::fs::write(&path, r#"{"identifier":"default"}"#).unwrap();

    let error = check_default_capability(&path).unwrap_err();

    assert!(error.contains("permissions array"));
}

fn capability(permissions: &str) -> (tempfile::TempDir, std::path::PathBuf) {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("default.json");
    std::fs::write(
        &path,
        format!(
            r#"{{
  "identifier": "default",
  "windows": ["main"],
  "permissions": {permissions}
}}"#
        ),
    )
    .unwrap();
    (directory, path)
}
