use dropsquash_core::{AppError, CaptureRect, Result};
use objc2::runtime::AnyObject;
use objc2_core_media::CMSampleBuffer;
use objc2_foundation::{NSNumber, NSString, NSValue};
use objc2_screen_capture_kit::{
    SCStreamFrameInfoBoundingRect, SCStreamFrameInfoContentRect, SCStreamFrameInfoContentScale,
    SCStreamFrameInfoDisplayTime, SCStreamFrameInfoScaleFactor, SCStreamFrameInfoStatus,
};

use super::{
    attachment_rect::capture_rect, ensure_required_sck_frame_attachments,
    SckFrameAttachmentPresence,
};

pub fn ensure_presence(sample_buffer: &CMSampleBuffer) -> Result<()> {
    ensure_required_sck_frame_attachments(SckFrameAttachmentPresence {
        status: has(sample_buffer, unsafe { SCStreamFrameInfoStatus }),
        display_time: has(sample_buffer, unsafe { SCStreamFrameInfoDisplayTime }),
        scale_factor: has(sample_buffer, unsafe { SCStreamFrameInfoScaleFactor }),
        content_scale: has(sample_buffer, unsafe { SCStreamFrameInfoContentScale }),
        content_rect: has(sample_buffer, unsafe { SCStreamFrameInfoContentRect }),
        bounding_rect: has(sample_buffer, unsafe { SCStreamFrameInfoBoundingRect }),
    })
}

pub fn status(sample_buffer: &CMSampleBuffer) -> Result<isize> {
    number(
        sample_buffer,
        unsafe { SCStreamFrameInfoStatus },
        |number| Ok(number.as_isize()),
    )
}

pub fn display_time(sample_buffer: &CMSampleBuffer) -> Result<u64> {
    let value = number(
        sample_buffer,
        unsafe { SCStreamFrameInfoDisplayTime },
        |number| Ok(number.as_i64()),
    )?;
    let ticks = u64::try_from(value).map_err(|_| {
        AppError::InvalidConfig("Secure Share SCK display time is negative".to_string())
    })?;
    super::mach_clock::nanoseconds(ticks)
}

pub fn scale_factor(sample_buffer: &CMSampleBuffer) -> Result<f32> {
    number(
        sample_buffer,
        unsafe { SCStreamFrameInfoScaleFactor },
        |number| Ok(number.as_f32()),
    )
}

pub fn content_scale(sample_buffer: &CMSampleBuffer) -> Result<f32> {
    number(
        sample_buffer,
        unsafe { SCStreamFrameInfoContentScale },
        |number| Ok(number.as_f32()),
    )
}

pub fn content_rect(sample_buffer: &CMSampleBuffer) -> Result<CaptureRect> {
    rect(sample_buffer, unsafe { SCStreamFrameInfoContentRect })
}

pub fn bounding_rect(sample_buffer: &CMSampleBuffer) -> Result<CaptureRect> {
    rect(sample_buffer, unsafe { SCStreamFrameInfoBoundingRect })
}

fn has(sample_buffer: &CMSampleBuffer, key: &NSString) -> bool {
    super::sample_attachments::has(sample_buffer, key)
}

fn number<T>(
    sample_buffer: &CMSampleBuffer,
    key: &NSString,
    read: impl FnOnce(&NSNumber) -> Result<T>,
) -> Result<T> {
    let object = required_attachment(sample_buffer, key)?;
    let number = object.downcast_ref::<NSNumber>().ok_or_else(|| {
        AppError::InvalidConfig("Secure Share SCK numeric attachment has wrong type".to_string())
    })?;
    read(number)
}

fn rect(sample_buffer: &CMSampleBuffer, key: &NSString) -> Result<CaptureRect> {
    let object = required_attachment(sample_buffer, key)?;
    if let Some(rect) = object.downcast_ref::<NSValue>() {
        return capture_rect(unsafe { rect.rectValue() });
    }
    dictionary_rect(object).ok_or_else(|| {
        AppError::InvalidConfig(format!(
            "Secure Share SCK rect attachment has wrong type: {}",
            super::sample_attachments::describe(object)
        ))
    })
}

fn required_attachment<'a>(
    sample_buffer: &'a CMSampleBuffer,
    key: &NSString,
) -> Result<&'a AnyObject> {
    super::sample_attachments::value(sample_buffer, key).ok_or_else(|| {
        AppError::InvalidConfig("Secure Share SCK required attachment is missing".to_string())
    })
}

fn dictionary_rect(object: &AnyObject) -> Option<CaptureRect> {
    Some(CaptureRect {
        x: rect_i32(object, "X")?,
        y: rect_i32(object, "Y")?,
        width: rect_u32(object, "Width")?,
        height: rect_u32(object, "Height")?,
    })
}

fn rect_i32(object: &AnyObject, key: &str) -> Option<i32> {
    let value = super::sample_attachments::dictionary_number(object, key)?;
    (value.is_finite() && value >= i32::MIN as f64 && value <= i32::MAX as f64)
        .then(|| value.round() as i32)
}

fn rect_u32(object: &AnyObject, key: &str) -> Option<u32> {
    let value = super::sample_attachments::dictionary_number(object, key)?;
    (value.is_finite() && value > 0.0 && value <= u32::MAX as f64).then(|| value.round() as u32)
}
