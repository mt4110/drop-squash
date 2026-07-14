use std::fs;

use tempfile::TempDir;

use super::copy_app_bundle;

#[test]
fn copies_app_bundle_without_replacing_existing_install() {
    let temp = TempDir::new().unwrap();
    let source = temp.path().join("DropSquash.app");
    let apps = temp.path().join("Applications");
    fs::create_dir_all(source.join("Contents/MacOS")).unwrap();
    fs::create_dir(&apps).unwrap();
    fs::write(source.join("Contents/MacOS/DropSquash"), "binary").unwrap();

    let install = copy_app_bundle(&source, &apps).unwrap();

    assert_eq!(install.target_path, apps.join("DropSquash.app"));
    assert!(install
        .target_path
        .join("Contents/MacOS/DropSquash")
        .exists());
    assert!(copy_app_bundle(&source, &apps).is_err());
}

#[test]
fn rejects_non_app_sources() {
    let temp = TempDir::new().unwrap();
    let source = temp.path().join("DropSquash");
    let apps = temp.path().join("Applications");
    fs::create_dir(&source).unwrap();
    fs::create_dir(&apps).unwrap();

    let error = copy_app_bundle(&source, &apps).unwrap_err().to_string();

    assert!(error.contains(".app"));
}
