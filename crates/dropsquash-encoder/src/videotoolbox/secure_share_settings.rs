use dropsquash_core::{AppError, Result};
use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2_av_foundation::{AVVideoCodecKey, AVVideoCodecTypeH264};
use objc2_core_video::{kCVPixelBufferPixelFormatTypeKey, kCVPixelFormatType_32BGRA};
use objc2_foundation::{ns_string, NSCopying, NSDictionary, NSNumber, NSString};

pub(super) fn reader_output_settings() -> Retained<NSDictionary<NSString, AnyObject>> {
    NSDictionary::from_retained_objects(
        &[pixel_format_key()],
        &[NSNumber::new_u32(kCVPixelFormatType_32BGRA).into()],
    )
}

pub(super) fn writer_output_settings() -> Result<Retained<NSDictionary<NSString, AnyObject>>> {
    let codec_key = unsafe { AVVideoCodecKey }.ok_or_else(|| {
        AppError::Encoder("H.264 output key is unavailable on this system".to_string())
    })?;
    let codec = unsafe { AVVideoCodecTypeH264 }.ok_or_else(|| {
        AppError::Encoder("H.264 output is unavailable on this system".to_string())
    })?;
    Ok(NSDictionary::from_retained_objects(
        &[codec_key],
        &[codec.copy().into()],
    ))
}

fn pixel_format_key() -> &'static NSString {
    let _ = unsafe { kCVPixelBufferPixelFormatTypeKey };
    ns_string!("PixelFormatType")
}
