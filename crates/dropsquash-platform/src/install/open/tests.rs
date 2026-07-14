use std::path::Path;

use super::validate_installed_app_path;

#[test]
fn accepts_direct_applications_app_bundle() {
    assert!(validate_installed_app_path(Path::new("/Applications/DropSquash.app")).is_ok());
}

#[test]
fn rejects_nested_applications_path() {
    let error = validate_installed_app_path(Path::new("/Applications/DropSquash.app/Contents"))
        .unwrap_err();

    assert!(error
        .to_string()
        .contains("direct /Applications .app bundle"));
}

#[test]
fn rejects_non_applications_path() {
    let error =
        validate_installed_app_path(Path::new("/Volumes/DropSquash/DropSquash.app")).unwrap_err();

    assert!(error
        .to_string()
        .contains("direct /Applications .app bundle"));
}
