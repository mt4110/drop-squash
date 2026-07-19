use dropsquash_core::{AppError, Result};
use objc2_core_media::CMSampleBuffer;
use objc2_core_video::{
    kCVPixelFormatType_32BGRA, kCVReturnSuccess, CVPixelBuffer, CVPixelBufferGetBaseAddress,
    CVPixelBufferGetBytesPerRow, CVPixelBufferGetHeight, CVPixelBufferGetPixelFormatType,
    CVPixelBufferGetWidth, CVPixelBufferLockBaseAddress, CVPixelBufferLockFlags,
    CVPixelBufferUnlockBaseAddress,
};

pub(crate) struct RowSignals {
    pub width: u32,
    pub height: u32,
    pub brightness: Vec<u8>,
    pub edge: Vec<u8>,
}

pub(crate) fn read_sample_buffer_row_signals(sample: &CMSampleBuffer) -> Result<RowSignals> {
    let image = unsafe { sample.image_buffer() }.ok_or_else(|| {
        AppError::UnsupportedMedia("Secure Share requires decoded video frames".into())
    })?;
    if CVPixelBufferGetPixelFormatType(&image) != kCVPixelFormatType_32BGRA {
        return Err(AppError::UnsupportedMedia(
            "Secure Share currently requires 32BGRA decoded frames".into(),
        ));
    }
    let flags = CVPixelBufferLockFlags(0);
    if unsafe { CVPixelBufferLockBaseAddress(&image, flags) } != kCVReturnSuccess {
        return Err(AppError::Encoder(
            "Secure Share could not lock the pixel buffer".into(),
        ));
    }
    let signals = read_locked_row_signals(&image);
    let _ = unsafe { CVPixelBufferUnlockBaseAddress(&image, flags) };
    signals
}

fn read_locked_row_signals(pixel: &CVPixelBuffer) -> Result<RowSignals> {
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
        .ok_or_else(|| AppError::InvalidConfig("secure-share frame shape mismatch".into()))?;
    let frame = unsafe { std::slice::from_raw_parts(base, len) };
    let mut brightness = vec![0; height as usize];
    let mut edge = vec![0; height as usize];
    for y in 0..height as usize {
        (brightness[y], edge[y]) = row_signal(frame, row, width as usize, y);
    }
    Ok(RowSignals {
        width,
        height,
        brightness,
        edge,
    })
}

fn row_signal(frame: &[u8], row: usize, width: usize, y: usize) -> (u8, u8) {
    let mut total = 0u32;
    let mut diffs = 0u32;
    let mut count = 0u32;
    let mut previous = None;
    for x in (0..width).step_by(16) {
        let at = y * row + x * 4;
        let gray =
            ((frame[at] as u32 * 29 + frame[at + 1] as u32 * 150 + frame[at + 2] as u32 * 77) >> 8)
                as u8;
        total += gray as u32;
        diffs += previous.map_or(0, |last: u8| gray.abs_diff(last) as u32);
        previous = Some(gray);
        count += 1;
    }
    (
        (total / count.max(1)) as u8,
        (diffs / count.saturating_sub(1).max(1)) as u8,
    )
}
