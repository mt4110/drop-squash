use std::slice;

use dropsquash_core::{AppError, MaskRect, PixelRect, Result};
use objc2_core_media::CMSampleBuffer;
use objc2_core_video::{
    kCVPixelFormatType_32BGRA, kCVReturnSuccess, CVPixelBufferGetBytesPerRow,
    CVPixelBufferGetHeight, CVPixelBufferGetPixelFormatType, CVPixelBufferGetWidth,
    CVPixelBufferLockBaseAddress, CVPixelBufferLockFlags, CVPixelBufferUnlockBaseAddress,
};

mod copy;
mod regions;
mod surface;
pub(crate) use copy::copied_blackened;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SckLiveMaskEvidence {
    pub masked_frame_count: usize,
    pub masked_rect_count: usize,
    pub verified_pixel_count: usize,
    pub vision_frame_count: usize,
    pub vision_total_ns: u64,
    pub vision_max_ns: u64,
}

impl SckLiveMaskEvidence {
    pub(super) fn record(&mut self, proof: FrameMaskProof) {
        self.masked_frame_count += usize::from(proof.rect_count > 0);
        self.masked_rect_count += proof.rect_count;
        self.verified_pixel_count += usize::from(proof.first_pixel_black);
    }

    pub(super) fn record_vision(&mut self, elapsed: std::time::Duration) {
        let elapsed_ns = elapsed.as_nanos().min(u128::from(u64::MAX)) as u64;
        self.vision_frame_count += 1;
        self.vision_total_ns = self.vision_total_ns.saturating_add(elapsed_ns);
        self.vision_max_ns = self.vision_max_ns.max(elapsed_ns);
    }

    pub(super) fn vision_summary(&self) -> String {
        if self.vision_frame_count == 0 {
            return String::new();
        }
        format!(
            "; local Vision analyzed {} frames, total {}ns, max {}ns",
            self.vision_frame_count, self.vision_total_ns, self.vision_max_ns
        )
    }
}

pub(crate) struct FrameMaskProof {
    rect_count: usize,
    first_pixel_black: bool,
}

pub(super) fn blacken_regions(
    sample: &CMSampleBuffer,
    regions: &[PixelRect],
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
    let result = surface::locked_base(&image)
        .and_then(|base| blacken_locked(&image, base.as_ptr(), regions));
    let _ = unsafe { CVPixelBufferUnlockBaseAddress(&image, flags) };
    result
}

pub(super) fn blacken_locked(
    pixel: &objc2_core_video::CVPixelBuffer,
    base: *mut u8,
    regions: &[PixelRect],
) -> Result<FrameMaskProof> {
    let width = CVPixelBufferGetWidth(pixel) as u32;
    let height = CVPixelBufferGetHeight(pixel) as u32;
    let row = CVPixelBufferGetBytesPerRow(pixel);
    if base.is_null() {
        return Err(AppError::Encoder(
            "Secure Share pixels are unavailable".into(),
        ));
    }
    let len = row
        .checked_mul(height as usize)
        .ok_or_else(|| AppError::InvalidConfig("Secure Share frame size is invalid".into()))?;
    let bytes = unsafe { slice::from_raw_parts_mut(base, len) };
    let rects = regions::expanded(regions, width, height);
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
