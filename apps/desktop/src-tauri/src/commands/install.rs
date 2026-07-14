use dropsquash_platform::{ApplicationsInstall, InstallLocation, InstallerCleanup};
use serde::Serialize;

use super::format_error;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallLocationDto {
    pub app_path: String,
    pub running_from_disk_image: bool,
    pub installed_in_applications: bool,
    pub should_offer_applications_move: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationsInstallDto {
    pub source_path: String,
    pub target_path: String,
    pub cleanup: InstallerCleanupDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallerCleanupDto {
    pub mounted_volume_path: Option<String>,
    pub downloaded_dmg_path: Option<String>,
    pub should_offer_mounted_volume_eject: bool,
    pub should_offer_downloaded_dmg_trash: bool,
}

pub fn load_install_location() -> Result<InstallLocationDto, String> {
    dropsquash_platform::current_install_location()
        .map(InstallLocationDto::from)
        .map_err(format_error)
}

pub fn copy_to_applications() -> Result<ApplicationsInstallDto, String> {
    dropsquash_platform::copy_current_app_to_applications()
        .map(ApplicationsInstallDto::from)
        .map_err(format_error)
}

impl From<InstallLocation> for InstallLocationDto {
    fn from(location: InstallLocation) -> Self {
        Self {
            app_path: location.app_path.display().to_string(),
            running_from_disk_image: location.running_from_disk_image,
            installed_in_applications: location.installed_in_applications,
            should_offer_applications_move: location.should_offer_applications_move,
        }
    }
}

impl From<ApplicationsInstall> for ApplicationsInstallDto {
    fn from(install: ApplicationsInstall) -> Self {
        Self {
            source_path: install.source_path.display().to_string(),
            target_path: install.target_path.display().to_string(),
            cleanup: InstallerCleanupDto::from(install.cleanup),
        }
    }
}

impl From<InstallerCleanup> for InstallerCleanupDto {
    fn from(cleanup: InstallerCleanup) -> Self {
        Self {
            mounted_volume_path: cleanup
                .mounted_volume_path
                .map(|path| path.display().to_string()),
            downloaded_dmg_path: cleanup
                .downloaded_dmg_path
                .map(|path| path.display().to_string()),
            should_offer_mounted_volume_eject: cleanup.should_offer_mounted_volume_eject,
            should_offer_downloaded_dmg_trash: cleanup.should_offer_downloaded_dmg_trash,
        }
    }
}

#[cfg(test)]
mod tests {
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
    fn dto_preserves_applications_install_paths() {
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
}
