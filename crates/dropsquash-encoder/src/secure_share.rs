use std::path::Path;

use dropsquash_core::{
    mask_options_for_frame, AppError, MaskMode, MaskPlan, MaskRect, Result, SecureShareOptions,
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MaskPlanBlackFillProof {
    pub frame_index: u64,
    pub frame_width: u32,
    pub frame_height: u32,
    pub mask_rect_count: usize,
    pub first_rect: MaskRect,
    pub first_sample_blackened: bool,
}

pub fn resolve_secure_share_options(
    input_path: &Path,
    options: Option<&SecureShareOptions>,
) -> Result<Option<SecureShareOptions>> {
    match options {
        None => Ok(None),
        Some(options) if !options.mask_rects.is_empty() => Ok(Some(options.clone())),
        Some(options) => resolve_auto_detected(input_path, options).map(Some),
    }
}

#[cfg(target_os = "macos")]
pub fn prove_mask_plan_solid_black_fill(plan: &MaskPlan) -> Result<MaskPlanBlackFillProof> {
    let frame = plan
        .frames
        .first()
        .ok_or_else(|| proof_error("MaskPlan has no frames"))?;
    let options = mask_options_for_frame(plan, frame.frame_index, MaskMode::SolidBlack)
        .ok_or_else(|| proof_error("MaskPlan frame is missing"))?;
    let first_rect = *options
        .mask_rects
        .first()
        .ok_or_else(|| proof_error("MaskPlan frame has no destructive rects"))?;
    let mut pixels = proof_pixels(plan.frame_size.width, plan.frame_size.height)?;
    crate::videotoolbox::apply_destructive_mask_rgba_for_proof(
        &mut pixels,
        plan.frame_size.width,
        plan.frame_size.height,
        &options,
    )?;
    Ok(MaskPlanBlackFillProof {
        frame_index: frame.frame_index,
        frame_width: plan.frame_size.width,
        frame_height: plan.frame_size.height,
        mask_rect_count: options.mask_rects.len(),
        first_rect,
        first_sample_blackened: is_black_at(&pixels, plan.frame_size.width, first_rect),
    })
}

#[cfg(not(target_os = "macos"))]
pub fn prove_mask_plan_solid_black_fill(_plan: &MaskPlan) -> Result<MaskPlanBlackFillProof> {
    Err(AppError::UnsupportedMedia(
        "MaskPlan black-fill proof is currently macOS-only".to_string(),
    ))
}

fn proof_pixels(width: u32, height: u32) -> Result<Vec<u8>> {
    let len = (width as usize)
        .checked_mul(height as usize)
        .and_then(|pixels| pixels.checked_mul(4))
        .ok_or_else(|| proof_error("MaskPlan proof frame is too large"))?;
    Ok(vec![200u8; len])
}

fn is_black_at(frame: &[u8], width: u32, rect: MaskRect) -> bool {
    let offset = ((rect.y * width + rect.x) * 4) as usize;
    frame.get(offset..offset + 4) == Some(&[0, 0, 0, 255])
}

fn proof_error(message: &str) -> AppError {
    AppError::InvalidConfig(format!(
        "Secure Share MaskPlan black-fill proof failed: {message}"
    ))
}

#[cfg(target_os = "macos")]
fn resolve_auto_detected(
    input_path: &Path,
    options: &SecureShareOptions,
) -> Result<SecureShareOptions> {
    Ok(SecureShareOptions {
        mask_mode: options.mask_mode,
        mask_rects: crate::videotoolbox::detect_auto_mask_rects(input_path)?,
    })
}

#[cfg(not(target_os = "macos"))]
fn resolve_auto_detected(
    _input_path: &Path,
    _options: &SecureShareOptions,
) -> Result<SecureShareOptions> {
    Err(AppError::UnsupportedMedia(
        "automatic fixed-bar masking is currently only available on macOS".to_string(),
    ))
}
