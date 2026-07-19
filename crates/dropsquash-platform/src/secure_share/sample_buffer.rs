use dropsquash_core::Result;
use objc2_core_media::CMSampleBuffer;
use objc2_screen_capture_kit::SCFrameStatus;

use super::{attachment_value, RawSckFrameInfo, RawSckFrameStatus};

pub fn raw_info_from_sample_buffer(
    sample_buffer: &CMSampleBuffer,
    frame_index: u64,
) -> Result<RawSckFrameInfo> {
    attachment_value::ensure_presence(sample_buffer)?;
    Ok(RawSckFrameInfo {
        frame_index,
        presentation_time_ns: attachment_value::display_time(sample_buffer)?,
        status: map_status(attachment_value::status(sample_buffer)?),
        content_rect: attachment_value::content_rect(sample_buffer)?,
        bounding_rect: attachment_value::bounding_rect(sample_buffer)?,
        scale_factor: attachment_value::scale_factor(sample_buffer)?,
        content_scale: attachment_value::content_scale(sample_buffer)?,
    })
}

fn map_status(status: isize) -> RawSckFrameStatus {
    match SCFrameStatus(status) {
        SCFrameStatus::Complete => RawSckFrameStatus::Complete,
        SCFrameStatus::Idle => RawSckFrameStatus::Idle,
        SCFrameStatus::Blank => RawSckFrameStatus::Blank,
        SCFrameStatus::Started => RawSckFrameStatus::Started,
        SCFrameStatus::Suspended => RawSckFrameStatus::Suspended,
        SCFrameStatus::Stopped => RawSckFrameStatus::Stopped,
        _ => RawSckFrameStatus::Unknown,
    }
}
