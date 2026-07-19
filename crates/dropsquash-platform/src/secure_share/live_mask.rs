use std::slice;

use dropsquash_core::{AppError, MaskRect, Result, VisionObservation};
use objc2_core_media::CMSampleBuffer;
use objc2_core_video::{
    kCVPixelFormatType_32BGRA, kCVReturnSuccess, CVPixelBufferGetBaseAddress,
    CVPixelBufferGetBytesPerRow, CVPixelBufferGetHeight, CVPixelBufferGetPixelFormatType,
    CVPixelBufferGetWidth, CVPixelBufferLockBaseAddress, CVPixelBufferLockFlags,
    CVPixelBufferUnlockBaseAddress,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SckLiveMaskEvidence {
    pub masked_frame_count: usize,
    pub masked_rect_count: usize,
    pub verified_pixel_count: usize,
}

impl SckLiveMaskEvidence {
    pub(super) fn record(&mut self, proof: FrameMaskProof) {
        self.masked_frame_count += usize::from(proof.rect_count > 0);
        self.masked_rect_count += proof.rect_count;
        self.verified_pixel_count += usize::from(proof.first_pixel_black);
    }
}

pub(super) struct FrameMaskProof {
    rect_count: usize,
    first_pixel_black: bool,
}

pub(super) fn blacken_observed_text(
    sample: &CMSampleBuffer,
    observations: &[VisionObservation],
) -> Result<FrameMaskProof> {
    let image = unsafe { sample.image_buffer() }
        .ok_or_else(|| AppError::UnsupportedMedia("Secure Share requires frame pixels".into()))?;
    if CVPixelBufferGetPixelFormatType(&image) != kCVPixelFormatType_32BGRA {
        return Err(AppError::UnsupportedMedia(
            "Secure Share requires 32BGRA frames".into(),
        ));
    }
    let flags = CVPixelBufferLockFlags(0);
    if unsafe { CVPixelBufferLockBaseAddress(&image, flags) } != kCVReturnSuccess {
        return Err(AppError::Encoder(
            "Secure Share could not lock captured pixels".into(),
        ));
    }
    let result = blacken_locked(&image, observations);
    let _ = unsafe { CVPixelBufferUnlockBaseAddress(&image, flags) };
    result
}

fn blacken_locked(
    pixel: &objc2_core_video::CVPixelBuffer,
    observations: &[VisionObservation],
) -> Result<FrameMaskProof> {
    let width = CVPixelBufferGetWidth(pixel) as u32;
    let height = CVPixelBufferGetHeight(pixel) as u32;
    let row = CVPixelBufferGetBytesPerRow(pixel);
    let base = CVPixelBufferGetBaseAddress(pixel).cast::<u8>();
    if base.is_null() {
        return Err(AppError::Encoder(
            "Secure Share pixels are unavailable".into(),
        ));
    }
    let len = row
        .checked_mul(height as usize)
        .ok_or_else(|| AppError::InvalidConfig("Secure Share frame size is invalid".into()))?;
    let bytes = unsafe { slice::from_raw_parts_mut(base, len) };
    let rects = observations
        .iter()
        .filter_map(|item| expand(item.rect, width, height))
        .collect::<Vec<_>>();
    for rect in &rects {
        fill_black(bytes, row, *rect);
    }
    let first_pixel_black = rects
        .first()
        .is_some_and(|rect| black_at(bytes, row, *rect));
    Ok(FrameMaskProof {
        rect_count: rects.len(),
        first_pixel_black,
    })
}

fn expand(rect: dropsquash_core::PixelRect, width: u32, height: u32) -> Option<MaskRect> {
    let x = rect.x.saturating_sub(8);
    let y = rect.y.saturating_sub(8);
    let right = rect
        .x
        .saturating_add(rect.width)
        .saturating_add(8)
        .min(width);
    let bottom = rect
        .y
        .saturating_add(rect.height)
        .saturating_add(8)
        .min(height);
    (right > x && bottom > y).then_some(MaskRect {
        x,
        y,
        width: right - x,
        height: bottom - y,
    })
}

fn fill_black(bytes: &mut [u8], row: usize, rect: MaskRect) {
    for y in rect.y..rect.y + rect.height {
        for x in rect.x..rect.x + rect.width {
            let at = y as usize * row + x as usize * 4;
            bytes[at..at + 4].copy_from_slice(&[0, 0, 0, 255]);
        }
    }
}

fn black_at(bytes: &[u8], row: usize, rect: MaskRect) -> bool {
    let at = rect.y as usize * row + rect.x as usize * 4;
    bytes[at..at + 3] == [0, 0, 0]
}
