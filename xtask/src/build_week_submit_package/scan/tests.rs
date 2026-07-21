use super::package;
use std::path::Path;

#[test]
fn accepts_plain_submit_package() {
    let dir = tempfile::tempdir().unwrap();
    write(dir.path(), "README.md", "DropSquash evidence package");
    write(
        dir.path(),
        "docs/build-week-final-entry-sheet.md",
        "local first",
    );
    package(dir.path()).unwrap();
}

#[test]
fn rejects_private_key_file_name() {
    let dir = tempfile::tempdir().unwrap();
    write(dir.path(), "docs/AuthKey_TEST.p8", "placeholder");
    let error = package(dir.path()).unwrap_err();
    assert!(error.contains("forbidden path marker"));
}

#[test]
fn rejects_secret_like_text() {
    let dir = tempfile::tempdir().unwrap();
    write(dir.path(), "docs/evidence.md", "APPLE_API_KEY=secret");
    let error = package(dir.path()).unwrap_err();
    assert!(error.contains("secret-like value"));
}

#[test]
fn skips_binary_dmg_body() {
    let dir = tempfile::tempdir().unwrap();
    write(
        dir.path(),
        "DropSquash_0.1.0_aarch64.dmg",
        "APPLE_API_KEY=not-scanned-inside-dmg",
    );
    package(dir.path()).unwrap();
}

fn write(root: &Path, path: &str, text: &str) {
    let path = root.join(path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(path, text).unwrap();
}
