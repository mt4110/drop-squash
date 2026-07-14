use dropsquash_platform::{ApplicationsInstall, InstallLocation, InstallerCleanup};
use serde::Serialize;

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
mod tests;
