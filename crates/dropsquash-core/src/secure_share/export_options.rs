use super::{FrameMaskPlan, FrameSize, MaskPlan, PixelRect, RegionPolicy};
use crate::{MaskMode, MaskRect, SecureShareOptions};

pub fn mask_options_for_frame(
    plan: &MaskPlan,
    frame_index: u64,
    mask_mode: MaskMode,
) -> Option<SecureShareOptions> {
    let frame = plan
        .frames
        .iter()
        .find(|frame| frame.frame_index == frame_index)?;
    Some(SecureShareOptions {
        mask_mode,
        mask_rects: frame_mask_rects(frame, plan.frame_size.width, plan.frame_size.height),
        mask_plan: None,
    })
}

fn frame_mask_rects(frame: &FrameMaskPlan, width: u32, height: u32) -> Vec<MaskRect> {
    effective_rects(frame, FrameSize { width, height })
        .into_iter()
        .map(|rect| MaskRect {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
        })
        .collect()
}

pub(crate) fn effective_rects(frame: &FrameMaskPlan, size: FrameSize) -> Vec<PixelRect> {
    frame
        .regions
        .iter()
        .filter(|region| region.policy != RegionPolicy::Safe)
        .filter_map(|region| expanded(region.rect, region.expansion_px, size.width, size.height))
        .collect()
}

fn expanded(rect: PixelRect, amount: u32, width: u32, height: u32) -> Option<PixelRect> {
    let left = rect.x.saturating_sub(amount);
    let top = rect.y.saturating_sub(amount);
    let right = rect.x.saturating_add(rect.width).saturating_add(amount);
    let bottom = rect.y.saturating_add(rect.height).saturating_add(amount);
    let right = right.min(width);
    let bottom = bottom.min(height);
    if right <= left || bottom <= top {
        return None;
    }
    Some(PixelRect {
        x: left,
        y: top,
        width: right - left,
        height: bottom - top,
    })
}
