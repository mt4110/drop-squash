use dropsquash_core::{AppError, MaskRect, Result, SecureShareOptions};
use objc2_core_media::CMSampleBuffer;
use objc2_core_video::{
    kCVPixelFormatType_32BGRA, kCVReturnSuccess, CVPixelBufferGetBaseAddress,
    CVPixelBufferGetBytesPerRow, CVPixelBufferGetHeight, CVPixelBufferGetPixelFormatType,
    CVPixelBufferGetWidth, CVPixelBufferLockBaseAddress, CVPixelBufferLockFlags,
    CVPixelBufferUnlockBaseAddress,
};

const MAX_BLACK_CHANNEL: u8 = 18;
// H.264 chroma quantization can turn encoded black into values such as 0,5,0.
const MAX_BLACK_CHANNEL_SPREAD: u8 = 6;

pub(crate) fn verify_black_regions(
    sample: &CMSampleBuffer,
    options: &SecureShareOptions,
) -> Result<()> {
    let pixel = unsafe { sample.image_buffer() }.ok_or_else(missing_pixels)?;
    if CVPixelBufferGetPixelFormatType(&pixel) != kCVPixelFormatType_32BGRA {
        return Err(AppError::UnsupportedMedia(
            "Secure Share verification requires 32BGRA decoded frames".to_string(),
        ));
    }
    let flags = CVPixelBufferLockFlags(0);
    if unsafe { CVPixelBufferLockBaseAddress(&pixel, flags) } != kCVReturnSuccess {
        return Err(AppError::Encoder(
            "Secure Share verification could not lock output pixels".to_string(),
        ));
    }
    let result = verify_locked(&pixel, options);
    let _ = unsafe { CVPixelBufferUnlockBaseAddress(&pixel, flags) };
    result
}

fn verify_locked(
    pixel: &objc2_core_video::CVPixelBuffer,
    options: &SecureShareOptions,
) -> Result<()> {
    let width = CVPixelBufferGetWidth(pixel) as u32;
    let height = CVPixelBufferGetHeight(pixel) as u32;
    let row = CVPixelBufferGetBytesPerRow(pixel);
    let base = CVPixelBufferGetBaseAddress(pixel).cast::<u8>();
    if base.is_null() {
        return Err(missing_pixels());
    }
    let len = row
        .checked_mul(height as usize)
        .ok_or_else(|| AppError::InvalidConfig("secure-share frame shape mismatch".to_string()))?;
    let bytes = unsafe { std::slice::from_raw_parts(base, len) };
    verify_bgra_regions(bytes, width, height, row, &options.mask_rects)
}

fn verify_bgra_regions(
    bytes: &[u8],
    width: u32,
    height: u32,
    row: usize,
    rects: &[MaskRect],
) -> Result<()> {
    let expected = row
        .checked_mul(height as usize)
        .ok_or_else(|| AppError::InvalidConfig("secure-share frame shape mismatch".to_string()))?;
    if row < width as usize * 4 || bytes.len() != expected {
        return Err(AppError::InvalidConfig(
            "secure-share frame shape mismatch".to_string(),
        ));
    }
    for rect in rects {
        for y in rect.y..rect.y.saturating_add(rect.height).min(height) {
            for x in rect.x..rect.x.saturating_add(rect.width).min(width) {
                let offset = y as usize * row + x as usize * 4;
                let rgba = &bytes[offset..offset + 4];
                if !is_neutral_black(rgba) {
                    return Err(AppError::Encoder(format!(
                        "Secure Share verification found non-black masked output pixels \\
                             at x={x}, y={y} (bgr={}, {}, {})",
                        rgba[0], rgba[1], rgba[2],
                    )));
                }
            }
        }
    }
    Ok(())
}

fn is_neutral_black(bgra: &[u8]) -> bool {
    let channels = &bgra[..3];
    let darkest = *channels.iter().min().expect("three BGR channels");
    let brightest = *channels.iter().max().expect("three BGR channels");
    brightest <= MAX_BLACK_CHANNEL && brightest - darkest <= MAX_BLACK_CHANNEL_SPREAD
}

fn missing_pixels() -> AppError {
    AppError::UnsupportedMedia("Secure Share verification requires decoded frames".to_string())
}

#[cfg(test)]
#[path = "verify_tests.rs"]
mod tests;
