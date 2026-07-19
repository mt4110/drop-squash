use dropsquash_core::{AppError, MaskMode, MaskRect, Result, SecureShareOptions};

#[allow(dead_code)]
pub(super) fn apply_destructive_mask_rgba(
    frame: &mut [u8],
    width: u32,
    height: u32,
    options: &SecureShareOptions,
    frame_seed: u64,
) -> Result<()> {
    apply_destructive_mask_rgba_stride(
        frame,
        width,
        height,
        width as usize * 4,
        options,
        frame_seed,
    )
}

pub(crate) fn apply_destructive_mask_rgba_for_proof(
    frame: &mut [u8],
    width: u32,
    height: u32,
    options: &SecureShareOptions,
) -> Result<()> {
    apply_destructive_mask_rgba(frame, width, height, options, 0)
}

#[allow(dead_code)]
pub(super) fn apply_destructive_mask_rgba_stride(
    frame: &mut [u8],
    width: u32,
    height: u32,
    bytes_per_row: usize,
    options: &SecureShareOptions,
    frame_seed: u64,
) -> Result<()> {
    let row_width = width as usize * 4;
    let expected = bytes_per_row
        .checked_mul(height as usize)
        .ok_or_else(|| AppError::InvalidConfig("secure-share frame shape mismatch".to_string()))?;
    if bytes_per_row < row_width || frame.len() != expected {
        return Err(AppError::InvalidConfig(
            "secure-share frame shape mismatch".to_string(),
        ));
    }
    for rect in &options.mask_rects {
        apply_rect(
            frame,
            width,
            height,
            bytes_per_row,
            *rect,
            options.mask_mode,
            frame_seed,
        );
    }
    Ok(())
}

#[allow(dead_code)]
fn apply_rect(
    frame: &mut [u8],
    width: u32,
    height: u32,
    bytes_per_row: usize,
    rect: MaskRect,
    mode: MaskMode,
    frame_seed: u64,
) {
    let x_end = rect.x.saturating_add(rect.width).min(width);
    let y_end = rect.y.saturating_add(rect.height).min(height);
    for y in rect.y.min(height)..y_end {
        for x in rect.x.min(width)..x_end {
            let offset = y as usize * bytes_per_row + x as usize * 4;
            let rgb = match mode {
                MaskMode::SolidBlack => [0, 0, 0],
                MaskMode::BlackNoise => noise_rgb(x, y, frame_seed),
            };
            frame[offset] = rgb[0];
            frame[offset + 1] = rgb[1];
            frame[offset + 2] = rgb[2];
            frame[offset + 3] = 255;
        }
    }
}

#[allow(dead_code)]
fn noise_rgb(x: u32, y: u32, frame_seed: u64) -> [u8; 3] {
    let mut value = frame_seed ^ ((x as u64) << 32) ^ (y as u64) ^ 0x9E37_79B9_7F4A_7C15;
    [
        next_noise(&mut value),
        next_noise(&mut value),
        next_noise(&mut value),
    ]
}

#[allow(dead_code)]
fn next_noise(state: &mut u64) -> u8 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    (*state as u8) & 0x3f
}
