mod dto;

use super::format_error;
pub use dto::{ApplicationsInstallDto, InstallLocationDto};

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

pub fn eject_installer_volume(mounted_volume_path: String) -> Result<(), String> {
    dropsquash_platform::eject_mounted_volume(std::path::Path::new(&mounted_volume_path))
        .map_err(format_error)
}

#[cfg(test)]
mod tests;
