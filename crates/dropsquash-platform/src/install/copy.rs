use std::fs;
use std::path::{Path, PathBuf};

use dropsquash_core::{AppError, Result};

use super::current_install_location;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationsInstall {
    pub source_path: PathBuf,
    pub target_path: PathBuf,
}

pub fn copy_current_app_to_applications() -> Result<ApplicationsInstall> {
    let location = current_install_location()?;
    if !location.should_offer_applications_move {
        return Err(AppError::InvalidConfig(
            "DropSquash is not running from a disk image".to_string(),
        ));
    }
    copy_app_bundle(&location.app_path, Path::new("/Applications"))
}

pub fn copy_app_bundle(app_path: &Path, applications_dir: &Path) -> Result<ApplicationsInstall> {
    require_app_bundle(app_path)?;
    require_directory(applications_dir, "Applications folder")?;
    let target_path = applications_dir.join(file_name(app_path)?);
    if target_path.exists() {
        return Err(AppError::InvalidConfig(format!(
            "application already exists: {}",
            target_path.display()
        )));
    }
    copy_dir(app_path, &target_path)?;
    Ok(ApplicationsInstall {
        source_path: app_path.to_path_buf(),
        target_path,
    })
}

fn require_app_bundle(path: &Path) -> Result<()> {
    require_directory(path, "app bundle")?;
    if path.extension().is_some_and(|extension| extension == "app") {
        Ok(())
    } else {
        Err(AppError::InvalidConfig(format!(
            "app bundle must end with .app: {}",
            path.display()
        )))
    }
}

fn require_directory(path: &Path, label: &str) -> Result<()> {
    if path.is_dir() {
        Ok(())
    } else {
        Err(AppError::InvalidConfig(format!(
            "{label} is not a directory: {}",
            path.display()
        )))
    }
}

fn file_name(path: &Path) -> Result<&std::ffi::OsStr> {
    path.file_name().ok_or_else(|| {
        AppError::InvalidConfig(format!("app bundle has no file name: {}", path.display()))
    })
}

fn copy_dir(source: &Path, target: &Path) -> Result<()> {
    fs::create_dir(target)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let from = entry.path();
        let to = target.join(entry.file_name());
        copy_entry(&from, &to)?;
    }
    Ok(())
}

fn copy_entry(source: &Path, target: &Path) -> Result<()> {
    let metadata = fs::symlink_metadata(source)?;
    if metadata.is_dir() {
        copy_dir(source, target)
    } else if metadata.file_type().is_symlink() {
        copy_symlink(source, target)
    } else {
        fs::copy(source, target).map(|_| ()).map_err(AppError::from)
    }
}

#[cfg(unix)]
fn copy_symlink(source: &Path, target: &Path) -> Result<()> {
    std::os::unix::fs::symlink(fs::read_link(source)?, target).map_err(AppError::from)
}

#[cfg(not(unix))]
fn copy_symlink(source: &Path, target: &Path) -> Result<()> {
    fs::copy(fs::read_link(source)?, target)
        .map(|_| ())
        .map_err(AppError::from)
}
