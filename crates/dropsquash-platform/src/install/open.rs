use std::path::Path;

use dropsquash_core::{AppError, Result};

#[cfg(test)]
mod tests;

pub fn open_installed_application(path: &Path) -> Result<()> {
    validate_installed_app_path(path)?;
    open(path)
}

pub fn validate_installed_app_path(path: &Path) -> Result<()> {
    let normalized = path.to_string_lossy().replace('\\', "/");
    if normalized.starts_with("/Applications/")
        && normalized.matches('/').count() == 2
        && normalized.ends_with(".app")
    {
        return Ok(());
    }
    Err(AppError::InvalidConfig(format!(
        "installed app must be a direct /Applications .app bundle: {}",
        path.display()
    )))
}

#[cfg(target_os = "macos")]
fn open(path: &Path) -> Result<()> {
    use objc2_app_kit::NSWorkspace;
    use objc2_foundation::NSURL;

    let url = NSURL::from_file_path(path).ok_or_else(|| {
        AppError::InvalidConfig(format!("could not create app URL for {}", path.display()))
    })?;
    if NSWorkspace::sharedWorkspace().openURL(&url) {
        Ok(())
    } else {
        Err(AppError::InvalidConfig(format!(
            "could not open installed app: {}",
            path.display()
        )))
    }
}

#[cfg(not(target_os = "macos"))]
fn open(_path: &Path) -> Result<()> {
    Err(AppError::InvalidConfig(
        "installed app opening is only implemented on macOS".to_string(),
    ))
}
