use dropsquash_platform::InstallLocation;
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

pub fn load_install_location() -> Result<InstallLocationDto, String> {
    dropsquash_platform::current_install_location()
        .map(InstallLocationDto::from)
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use dropsquash_platform::InstallLocation;

    use super::InstallLocationDto;

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
}
