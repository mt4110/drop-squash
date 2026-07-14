use std::path::Path;

use super::cleanup_after_applications_install;

#[test]
fn offers_eject_after_installing_from_mounted_volume() {
    let cleanup = cleanup_after_applications_install(
        Path::new("/Volumes/DropSquash/DropSquash.app"),
        Path::new("/Applications/DropSquash.app"),
    );

    assert_eq!(
        cleanup.mounted_volume_path.as_deref(),
        Some(Path::new("/Volumes/DropSquash"))
    );
    assert!(cleanup.should_offer_mounted_volume_eject);
    assert!(cleanup.downloaded_dmg_path.is_none());
    assert!(!cleanup.should_offer_downloaded_dmg_trash);
}

#[test]
fn does_not_offer_download_cleanup_when_dmg_path_is_unproven() {
    let cleanup = cleanup_after_applications_install(
        Path::new("/Volumes/DropSquash/DropSquash.app"),
        Path::new("/Applications/DropSquash.app"),
    );

    assert!(cleanup.downloaded_dmg_path.is_none());
    assert!(!cleanup.should_offer_downloaded_dmg_trash);
}

#[test]
fn rejects_non_volume_source_for_eject_offer() {
    let cleanup = cleanup_after_applications_install(
        Path::new("/Users/me/DropSquash.app"),
        Path::new("/Applications/DropSquash.app"),
    );

    assert!(cleanup.mounted_volume_path.is_none());
    assert!(!cleanup.should_offer_mounted_volume_eject);
}
