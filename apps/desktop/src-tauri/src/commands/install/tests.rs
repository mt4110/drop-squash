use super::eject_installer_volume;

#[test]
fn eject_rejects_non_volume_paths() {
    let error = eject_installer_volume("/Applications/DropSquash.app".to_string()).unwrap_err();

    assert!(error.contains("direct /Volumes mount"));
}
