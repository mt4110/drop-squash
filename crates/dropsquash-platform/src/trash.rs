use std::path::Path;

use dropsquash_core::{AppError, Result};

#[derive(Debug, Clone, Default)]
pub struct TrashService;

impl TrashService {
    pub fn move_to_trash(&self, path: &Path) -> Result<()> {
        if !path.exists() {
            return Err(AppError::InvalidConfig(format!(
                "source file does not exist: {}",
                path.display()
            )));
        }
        move_to_trash(path)
    }
}

#[cfg(target_os = "macos")]
fn move_to_trash(path: &Path) -> Result<()> {
    use objc2_foundation::{NSFileManager, NSURL};

    let url = NSURL::from_file_path(path).ok_or_else(|| {
        AppError::InvalidConfig(format!("could not create file URL for {}", path.display()))
    })?;
    let manager = NSFileManager::defaultManager();
    manager
        .trashItemAtURL_resultingItemURL_error(&url, None)
        .map_err(|error| AppError::InvalidConfig(error.localizedDescription().to_string()))
}

#[cfg(not(target_os = "macos"))]
fn move_to_trash(_path: &Path) -> Result<()> {
    Err(AppError::InvalidConfig(
        "trash integration is only implemented on macOS".to_string(),
    ))
}

#[cfg(test)]
mod tests;
