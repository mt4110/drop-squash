use dropsquash_core::{
    mask_options_for_frame, AppError, FrameSize, MaskMode, MaskPolicy, Result, SecureShareOptions,
};

pub(crate) fn options_for_frame(
    options: &SecureShareOptions,
    frame_index: u64,
    frame_size: FrameSize,
) -> Result<SecureShareOptions> {
    if options.mask_mode != MaskMode::SolidBlack {
        return Err(invalid("verified export currently requires Solid Black"));
    }
    let Some(plan) = &options.mask_plan else {
        return fixed_options(options);
    };
    if plan.policy != MaskPolicy::StrictReveal {
        return Err(invalid("MaskPlan must use Strict Reveal"));
    }
    if plan.frame_size != frame_size {
        return Err(invalid(&format!(
            "MaskPlan frame size {}x{} does not match decoded video {}x{}",
            plan.frame_size.width, plan.frame_size.height, frame_size.width, frame_size.height
        )));
    }
    let resolved = mask_options_for_frame(plan, frame_index, options.mask_mode)
        .ok_or_else(|| invalid("MaskPlan is missing a decoded video frame"))?;
    if resolved.mask_rects.is_empty() {
        return Err(invalid("MaskPlan frame has no destructive regions"));
    }
    if !resolved
        .mask_rects
        .iter()
        .any(|rect| covers_frame(*rect, frame_size))
    {
        return Err(invalid("Strict Reveal frame is not fully covered"));
    }
    Ok(resolved)
}

fn covers_frame(rect: dropsquash_core::MaskRect, frame_size: FrameSize) -> bool {
    rect.x == 0 && rect.y == 0 && rect.width == frame_size.width && rect.height == frame_size.height
}

fn fixed_options(options: &SecureShareOptions) -> Result<SecureShareOptions> {
    if options.mask_rects.is_empty() {
        return Err(invalid("Secure Share requires destructive regions"));
    }
    Ok(SecureShareOptions {
        mask_mode: options.mask_mode,
        mask_rects: options.mask_rects.clone(),
        mask_plan: None,
    })
}

fn invalid(message: &str) -> AppError {
    AppError::InvalidConfig(format!("Secure Share export rejected: {message}"))
}
