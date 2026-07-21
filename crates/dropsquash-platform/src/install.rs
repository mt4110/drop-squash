use std::path::{Path, PathBuf};

use dropsquash_core::AppError;

mod cleanup;
mod copy;
mod eject;
mod open;
mod reveal;

pub use cleanup::{cleanup_after_applications_install, InstallerCleanup};
pub use copy::{copy_app_bundle, copy_current_app_to_applications, ApplicationsInstall};
pub use eject::{eject_mounted_volume, validate_mounted_volume_path};
pub use open::{open_installed_application, validate_installed_app_path};
pub use reveal::reveal_finder_item;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallLocation {
    pub app_path: PathBuf,
    pub running_from_disk_image: bool,
    pub installed_in_applications: bool,
    pub should_offer_applications_move: bool,
}

pub fn current_install_location() -> dropsquash_core::Result<InstallLocation> {
    let executable = std::env::current_exe().map_err(|error| {
        AppError::InvalidConfig(format!("could not read current executable path: {error}"))
    })?;
    Ok(install_location_for_executable(&executable))
}

pub fn install_location_for_executable(executable: &Path) -> InstallLocation {
    let app_path = app_bundle_path(executable);
    let running_from_disk_image = path_starts_with(&app_path, "/Volumes");
    let installed_in_applications = path_starts_with(&app_path, "/Applications");
    InstallLocation {
        app_path,
        running_from_disk_image,
        installed_in_applications,
        should_offer_applications_move: cfg!(target_os = "macos")
            && running_from_disk_image
            && !installed_in_applications,
    }
}

fn app_bundle_path(executable: &Path) -> PathBuf {
    executable
        .ancestors()
        .find(|path| path.extension().is_some_and(|extension| extension == "app"))
        .unwrap_or(executable)
        .to_path_buf()
}

fn path_starts_with(path: &Path, prefix: &str) -> bool {
    path.starts_with(Path::new(prefix))
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::install_location_for_executable;

    #[test]
    fn detects_app_launched_from_mounted_dmg() {
        let location = install_location_for_executable(Path::new(
            "/Volumes/DropSquash/DropSquash.app/Contents/MacOS/DropSquash",
        ));

        assert_eq!(
            location.app_path,
            Path::new("/Volumes/DropSquash/DropSquash.app")
        );
        assert!(location.running_from_disk_image);
        assert!(!location.installed_in_applications);
        assert_eq!(
            location.should_offer_applications_move,
            cfg!(target_os = "macos")
        );
    }

    #[test]
    fn detects_app_installed_in_applications() {
        let location = install_location_for_executable(Path::new(
            "/Applications/DropSquash.app/Contents/MacOS/DropSquash",
        ));

        assert!(location.installed_in_applications);
        assert!(!location.running_from_disk_image);
        assert!(!location.should_offer_applications_move);
    }

    #[test]
    fn falls_back_to_executable_without_app_bundle() {
        let executable = Path::new("/usr/local/bin/dropsquash");
        let location = install_location_for_executable(executable);

        assert_eq!(location.app_path, executable);
        assert!(!location.running_from_disk_image);
    }
}
