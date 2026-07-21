use std::path::Path;

use dropsquash_core::{AppError, Result};

#[cfg(test)]
mod tests;

pub fn reveal_finder_item(path: &Path) -> Result<()> {
    if !path.exists() {
        return Err(AppError::InvalidConfig(format!(
            "finder item does not exist: {}",
            path.display()
        )));
    }
    reveal(path)
}

#[cfg(target_os = "macos")]
fn reveal(path: &Path) -> Result<()> {
    use objc2_app_kit::NSWorkspace;
    use objc2_foundation::NSString;

    let path = NSString::from_str(&path.to_string_lossy());
    let root = NSString::from_str("");
    if NSWorkspace::sharedWorkspace().selectFile_inFileViewerRootedAtPath(Some(&path), &root) {
        return Ok(());
    }
    Err(AppError::InvalidConfig(format!(
        "could not reveal finder item: {}",
        path
    )))
}

#[cfg(not(target_os = "macos"))]
fn reveal(_path: &Path) -> Result<()> {
    Err(AppError::InvalidConfig(
        "Finder reveal is only implemented on macOS".to_string(),
    ))
}
