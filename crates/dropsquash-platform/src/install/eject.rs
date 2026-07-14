use std::path::Path;

use dropsquash_core::{AppError, Result};

#[cfg(test)]
mod tests;

pub fn eject_mounted_volume(path: &Path) -> Result<()> {
    validate_mounted_volume_path(path)?;
    eject(path)
}

pub fn validate_mounted_volume_path(path: &Path) -> Result<()> {
    let normalized = path.to_string_lossy().replace('\\', "/");
    if normalized.starts_with("/Volumes/") && normalized.matches('/').count() == 2 {
        return Ok(());
    }
    Err(AppError::InvalidConfig(format!(
        "installer volume must be a direct /Volumes mount: {}",
        path.display()
    )))
}

#[cfg(target_os = "macos")]
fn eject(path: &Path) -> Result<()> {
    use objc2_app_kit::NSWorkspace;
    use objc2_foundation::NSURL;

    let url = NSURL::from_file_path(path).ok_or_else(|| {
        AppError::InvalidConfig(format!(
            "could not create volume URL for {}",
            path.display()
        ))
    })?;
    NSWorkspace::sharedWorkspace()
        .unmountAndEjectDeviceAtURL_error(&url)
        .map_err(|error| AppError::InvalidConfig(error.to_string()))
}

#[cfg(not(target_os = "macos"))]
fn eject(_path: &Path) -> Result<()> {
    Err(AppError::InvalidConfig(
        "installer volume eject is only implemented on macOS".to_string(),
    ))
}
