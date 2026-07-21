use dropsquash_core::{AppError, FrameSize, Result, SecureShareOptions};
use objc2_core_media::CMSampleBuffer;
use objc2_core_video::{
    kCVPixelFormatType_32BGRA, kCVReturnSuccess, CVPixelBuffer, CVPixelBufferGetBaseAddress,
    CVPixelBufferGetBytesPerRow, CVPixelBufferGetHeight, CVPixelBufferGetPixelFormatType,
    CVPixelBufferGetWidth, CVPixelBufferLockBaseAddress, CVPixelBufferLockFlags,
    CVPixelBufferUnlockBaseAddress,
};

use super::mask::apply_destructive_mask_rgba_stride;

mod signals;
pub(crate) mod verify;
pub(super) use signals::{read_sample_buffer_row_signals, RowSignals};

pub(super) fn mask_sample_buffer(
    sample: &CMSampleBuffer,
    options: &SecureShareOptions,
    frame_seed: u64,
) -> Result<()> {
    let image = unsafe { sample.image_buffer() }.ok_or_else(|| {
        AppError::UnsupportedMedia("Secure Share requires decoded video frames".to_string())
    })?;
    mask_pixel_buffer(&image, options, frame_seed)
}

pub(super) fn sample_frame_size(sample: &CMSampleBuffer) -> Result<FrameSize> {
    let image = unsafe { sample.image_buffer() }.ok_or_else(|| {
        AppError::UnsupportedMedia("Secure Share requires decoded video frames".to_string())
    })?;
    Ok(FrameSize {
        width: CVPixelBufferGetWidth(&image) as u32,
        height: CVPixelBufferGetHeight(&image) as u32,
    })
}

fn mask_pixel_buffer(
    pixel: &CVPixelBuffer,
    options: &SecureShareOptions,
    frame_seed: u64,
) -> Result<()> {
    if CVPixelBufferGetPixelFormatType(pixel) != kCVPixelFormatType_32BGRA {
        return Err(AppError::UnsupportedMedia(
            "Secure Share currently requires 32BGRA decoded frames".to_string(),
        ));
    }
    let flags = CVPixelBufferLockFlags(0);
    let status = unsafe { CVPixelBufferLockBaseAddress(pixel, flags) };
    if status != kCVReturnSuccess {
        return Err(AppError::Encoder(format!(
            "Secure Share could not lock the pixel buffer ({status})"
        )));
    }
    let result = mask_locked_pixel_buffer(pixel, options, frame_seed);
    let _ = unsafe { CVPixelBufferUnlockBaseAddress(pixel, flags) };
    result
}

fn mask_locked_pixel_buffer(
    pixel: &CVPixelBuffer,
    options: &SecureShareOptions,
    frame_seed: u64,
) -> Result<()> {
    let width = CVPixelBufferGetWidth(pixel) as u32;
    let height = CVPixelBufferGetHeight(pixel) as u32;
    let bytes_per_row = CVPixelBufferGetBytesPerRow(pixel);
    let base = CVPixelBufferGetBaseAddress(pixel).cast::<u8>();
    if base.is_null() {
        return Err(AppError::Encoder(
            "Secure Share could not access the pixel buffer bytes".to_string(),
        ));
    }
    let len = bytes_per_row
        .checked_mul(height as usize)
        .ok_or_else(|| AppError::InvalidConfig("secure-share frame shape mismatch".to_string()))?;
    let frame = unsafe { std::slice::from_raw_parts_mut(base, len) };
    apply_destructive_mask_rgba_stride(frame, width, height, bytes_per_row, options, frame_seed)
}
