use dropsquash_core::{AppError, Result};

use super::RawSckFrameInfo;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SckFrameAttachmentPresence {
    pub status: bool,
    pub display_time: bool,
    pub scale_factor: bool,
    pub content_scale: bool,
    pub content_rect: bool,
    pub bounding_rect: bool,
}

impl SckFrameAttachmentPresence {
    pub fn complete() -> Self {
        Self {
            status: true,
            display_time: true,
            scale_factor: true,
            content_scale: true,
            content_rect: true,
            bounding_rect: true,
        }
    }
}

pub fn ensure_required_sck_frame_attachments(presence: SckFrameAttachmentPresence) -> Result<()> {
    let missing = missing_required_sck_frame_attachment(presence);
    if let Some(name) = missing {
        return Err(AppError::InvalidConfig(format!(
            "Secure Share ScreenCaptureKit frame metadata missing {name} attachment"
        )));
    }
    Ok(())
}

fn missing_required_sck_frame_attachment(
    presence: SckFrameAttachmentPresence,
) -> Option<&'static str> {
    if !presence.status {
        return Some("status");
    }
    if !presence.display_time {
        return Some("display time");
    }
    if !presence.scale_factor {
        return Some("scale factor");
    }
    if !presence.content_scale {
        return Some("content scale");
    }
    if !presence.content_rect {
        return Some("content rect");
    }
    if !presence.bounding_rect {
        return Some("bounding rect");
    }
    None
}

#[cfg(target_os = "macos")]
pub type NativeSampleBuffer = objc2_core_media::CMSampleBuffer;

#[cfg(not(target_os = "macos"))]
pub type NativeSampleBuffer = ();

#[cfg(target_os = "macos")]
pub fn raw_sck_frame_info_from_sample_buffer(
    sample_buffer: &NativeSampleBuffer,
    frame_index: u64,
) -> Result<RawSckFrameInfo> {
    super::sample_buffer::raw_info_from_sample_buffer(sample_buffer, frame_index)
}

#[cfg(not(target_os = "macos"))]
pub fn raw_sck_frame_info_from_sample_buffer(
    _sample_buffer: &NativeSampleBuffer,
    _frame_index: u64,
) -> Result<RawSckFrameInfo> {
    Err(AppError::UnsupportedMedia(
        "Secure Share sample-buffer attachment extraction is macOS-only".to_string(),
    ))
}

pub fn core_media_attachment_binding_name() -> &'static str {
    #[cfg(target_os = "macos")]
    {
        let _ = core::mem::size_of::<&NativeSampleBuffer>();
    }
    "objc2-core-media"
}
