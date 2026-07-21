use std::ptr::NonNull;

use dropsquash_core::{AppError, Result};
use objc2_core_foundation::CFRetained;
use objc2_core_video::{CVPixelBuffer, CVPixelBufferGetBaseAddress, CVPixelBufferGetIOSurface};
use objc2_io_surface::{IOSurfaceLockOptions, IOSurfaceRef};

pub(super) enum LockedBase {
    Pixel(NonNull<u8>),
    Surface(CFRetained<IOSurfaceRef>, NonNull<u8>),
}

impl LockedBase {
    pub(super) fn as_ptr(&self) -> *mut u8 {
        match self {
            Self::Pixel(base) | Self::Surface(_, base) => base.as_ptr(),
        }
    }
}

impl Drop for LockedBase {
    fn drop(&mut self) {
        if let Self::Surface(surface, _) = self {
            let _ = unsafe { surface.unlock(IOSurfaceLockOptions::empty(), std::ptr::null_mut()) };
        }
    }
}

pub(super) fn locked_base(pixel: &CVPixelBuffer) -> Result<LockedBase> {
    if let Some(base) = NonNull::new(CVPixelBufferGetBaseAddress(pixel).cast::<u8>()) {
        return Ok(LockedBase::Pixel(base));
    }
    let surface = CVPixelBufferGetIOSurface(Some(pixel)).ok_or_else(missing_surface)?;
    if unsafe { surface.lock(IOSurfaceLockOptions::empty(), std::ptr::null_mut()) } != 0 {
        return Err(lock_failed());
    }
    let base = surface.base_address().cast::<u8>();
    Ok(LockedBase::Surface(surface, base))
}

fn missing_surface() -> AppError {
    AppError::Encoder("Secure Share pixels are unavailable".into())
}

fn lock_failed() -> AppError {
    AppError::Encoder("Secure Share IOSurface lock failed".into())
}
