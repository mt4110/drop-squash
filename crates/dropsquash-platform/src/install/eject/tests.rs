use std::path::Path;

use super::validate_mounted_volume_path;

#[test]
fn accepts_direct_volume_mount() {
    assert!(validate_mounted_volume_path(Path::new("/Volumes/DropSquash")).is_ok());
}

#[test]
fn accepts_windows_style_test_path() {
    assert!(validate_mounted_volume_path(Path::new("\\Volumes\\DropSquash")).is_ok());
}

#[test]
fn rejects_nested_volume_path() {
    let error = validate_mounted_volume_path(Path::new("/Volumes/DropSquash/DropSquash.app"))
        .unwrap_err()
        .to_string();

    assert!(error.contains("direct /Volumes mount"));
}

#[test]
fn rejects_non_volume_path() {
    let error = validate_mounted_volume_path(Path::new("/Applications/DropSquash.app"))
        .unwrap_err()
        .to_string();

    assert!(error.contains("/Volumes"));
}
