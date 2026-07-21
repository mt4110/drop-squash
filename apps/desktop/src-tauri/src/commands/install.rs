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

pub fn open_installed_application(installed_app_path: String) -> Result<(), String> {
    dropsquash_platform::open_installed_application(std::path::Path::new(&installed_app_path))
        .map_err(format_error)
}

#[tauri::command(rename_all = "camelCase")]
pub fn reveal_finder_item(path: String) -> Result<(), String> {
    dropsquash_platform::reveal_finder_item(std::path::Path::new(&path)).map_err(format_error)
}

pub fn quit_after_installer_volume_eject(
    app: tauri::AppHandle,
    mounted_volume_path: String,
) -> Result<(), String> {
    std::process::Command::new(helper_executable()?)
        .args([
            "--dropsquash-eject-volume",
            &mounted_volume_path,
            "--dropsquash-wait-pid",
            &std::process::id().to_string(),
        ])
        .spawn()
        .map_err(|error| format!("failed to spawn eject helper: {error}"))?;
    app.exit(0);
    Ok(())
}

fn helper_executable() -> Result<std::path::PathBuf, String> {
    let current =
        std::env::current_exe().map_err(|error| format!("failed to locate helper: {error}"))?;
    let binary = current
        .file_name()
        .ok_or_else(|| "current executable has no file name".to_string())?;
    let app = current
        .ancestors()
        .find(|path| path.extension().is_some_and(|extension| extension == "app"))
        .and_then(std::path::Path::file_name)
        .ok_or_else(|| "current executable is not inside an app bundle".to_string())?;
    Ok(std::path::Path::new("/Applications")
        .join(app)
        .join("Contents")
        .join("MacOS")
        .join(binary))
}

#[cfg(test)]
mod tests;
