use std::ptr::NonNull;

use dropsquash_core::{AppError, PixelRect, Result};
use objc2_core_foundation::CFRetained;
use objc2_core_image::{CIContext, CIImage};
use objc2_core_media::CMSampleBuffer;
use objc2_core_video::{
    kCVPixelFormatType_32BGRA, kCVReturnSuccess, CVPixelBuffer, CVPixelBufferCreate,
    CVPixelBufferGetBaseAddress, CVPixelBufferGetHeight, CVPixelBufferGetWidth,
    CVPixelBufferLockBaseAddress, CVPixelBufferLockFlags, CVPixelBufferUnlockBaseAddress,
};

use super::{blacken_locked, FrameMaskProof};

pub(crate) struct CopiedFrame {
    pub(crate) pixel: CFRetained<CVPixelBuffer>,
    pub(crate) proof: FrameMaskProof,
}

pub(crate) fn copied_blackened(
    sample: &CMSampleBuffer,
    regions: &[PixelRect],
) -> Result<CopiedFrame> {
    let source = unsafe { sample.image_buffer() }.ok_or_else(missing)?;
    let target = cpu_pixels(&source)?;
    let context = unsafe { CIContext::contextWithOptions(None) };
    let image = unsafe { CIImage::imageWithCVPixelBuffer(&source) };
    unsafe { context.render_toCVPixelBuffer(&image, &target) };
    let flags = CVPixelBufferLockFlags(0);
    if unsafe { CVPixelBufferLockBaseAddress(&target, flags) } != kCVReturnSuccess {
        return Err(AppError::Encoder(
            "Secure Share could not lock rendered pixels".into(),
        ));
    }
    let result = blacken_locked(
        &target,
        CVPixelBufferGetBaseAddress(&target).cast(),
        regions,
    );
    let _ = unsafe { CVPixelBufferUnlockBaseAddress(&target, flags) };
    Ok(CopiedFrame {
        pixel: target,
        proof: result?,
    })
}

fn cpu_pixels(source: &CVPixelBuffer) -> Result<CFRetained<CVPixelBuffer>> {
    let mut raw = std::ptr::null_mut();
    let status = unsafe {
        CVPixelBufferCreate(
            None,
            CVPixelBufferGetWidth(source),
            CVPixelBufferGetHeight(source),
            kCVPixelFormatType_32BGRA,
            None,
            NonNull::from(&mut raw),
        )
    };
    if status != kCVReturnSuccess {
        return Err(AppError::Encoder(
            "Secure Share could not allocate output pixels".into(),
        ));
    }
    NonNull::new(raw)
        .map(|pixel| unsafe { CFRetained::from_raw(pixel) })
        .ok_or_else(missing)
}

fn missing() -> AppError {
    AppError::Encoder("Secure Share copied pixels are unavailable".into())
}
