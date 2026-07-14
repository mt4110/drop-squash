use std::path::PathBuf;

use dropsquash_platform::{ApplicationsInstall, InstallLocation, InstallerCleanup};

use super::{ApplicationsInstallDto, InstallLocationDto};

#[test]
fn dto_preserves_install_location_flags() {
    let dto = InstallLocationDto::from(InstallLocation {
        app_path: PathBuf::from("/Volumes/DropSquash/DropSquash.app"),
        running_from_disk_image: true,
        installed_in_applications: false,
        should_offer_applications_move: true,
    });

    assert_eq!(dto.app_path, "/Volumes/DropSquash/DropSquash.app");
    assert!(dto.running_from_disk_image);
    assert!(!dto.installed_in_applications);
    assert!(dto.should_offer_applications_move);
}

#[test]
fn dto_preserves_applications_install_cleanup() {
    let dto = ApplicationsInstallDto::from(ApplicationsInstall {
        source_path: PathBuf::from("/Volumes/DropSquash/DropSquash.app"),
        target_path: PathBuf::from("/Applications/DropSquash.app"),
        cleanup: InstallerCleanup {
            mounted_volume_path: Some(PathBuf::from("/Volumes/DropSquash")),
            downloaded_dmg_path: None,
            should_offer_mounted_volume_eject: true,
            should_offer_downloaded_dmg_trash: false,
        },
    });

    assert_eq!(dto.source_path, "/Volumes/DropSquash/DropSquash.app");
    assert_eq!(dto.target_path, "/Applications/DropSquash.app");
    assert_eq!(
        dto.cleanup.mounted_volume_path.as_deref(),
        Some("/Volumes/DropSquash")
    );
    assert!(dto.cleanup.should_offer_mounted_volume_eject);
    assert!(!dto.cleanup.should_offer_downloaded_dmg_trash);
}
