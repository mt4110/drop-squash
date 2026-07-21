use std::path::Path;

use dropsquash_core::{Result, SecureShareOptions};

#[cfg(target_os = "macos")]
pub fn verify_experimental_mask_plan_output(
    path: &Path,
    options: &SecureShareOptions,
) -> Result<()> {
    crate::videotoolbox::verify_experimental_masked_output(path, options)
}

#[cfg(not(target_os = "macos"))]
pub fn verify_experimental_mask_plan_output(
    _path: &Path,
    _options: &SecureShareOptions,
) -> Result<()> {
    Err(dropsquash_core::AppError::UnsupportedMedia(
        "experimental MaskPlan verification is currently macOS-only".to_string(),
    ))
}
