use super::{eject_installer_volume, open_installed_application};

#[test]
fn eject_rejects_non_volume_paths() {
    let error = eject_installer_volume("/Applications/DropSquash.app".to_string()).unwrap_err();

    assert!(error.contains("direct /Volumes mount"));
}

#[test]
fn open_installed_app_rejects_non_applications_paths() {
    let error =
        open_installed_application("/Volumes/DropSquash/DropSquash.app".to_string()).unwrap_err();

    assert!(error.contains("direct /Applications .app bundle"));
}
