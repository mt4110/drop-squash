use dropsquash_core::{AppError, CaptureFrameMetadata, CaptureRect, FrameStatus, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RawSckFrameInfo {
    pub frame_index: u64,
    pub presentation_time_ns: u64,
    pub status: RawSckFrameStatus,
    pub content_rect: CaptureRect,
    pub bounding_rect: CaptureRect,
    pub scale_factor: f32,
    pub content_scale: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawSckFrameStatus {
    Complete,
    Idle,
    Blank,
    Started,
    Suspended,
    Stopped,
    Unknown,
}

impl RawSckFrameInfo {
    pub fn into_metadata(self) -> Result<CaptureFrameMetadata> {
        validate_rect(self.content_rect, "content rect")?;
        validate_rect(self.bounding_rect, "bounding rect")?;
        validate_scale(self.scale_factor, "scale factor")?;
        validate_scale(self.content_scale, "content scale")?;
        Ok(CaptureFrameMetadata {
            frame_index: self.frame_index,
            presentation_time_ns: self.presentation_time_ns,
            frame_status: self.status.into(),
            content_rect: self.content_rect,
            bounding_rect: self.bounding_rect,
            scale_factor: self.scale_factor,
            content_scale: self.content_scale,
        })
    }
}

impl From<RawSckFrameStatus> for FrameStatus {
    fn from(value: RawSckFrameStatus) -> Self {
        match value {
            RawSckFrameStatus::Complete => Self::Complete,
            RawSckFrameStatus::Idle => Self::Idle,
            RawSckFrameStatus::Blank => Self::Blank,
            RawSckFrameStatus::Started => Self::Started,
            RawSckFrameStatus::Suspended => Self::Suspended,
            RawSckFrameStatus::Stopped => Self::Stopped,
            RawSckFrameStatus::Unknown => Self::Unknown,
        }
    }
}

fn validate_rect(rect: CaptureRect, name: &str) -> Result<()> {
    if rect.width == 0 || rect.height == 0 {
        return Err(AppError::InvalidConfig(format!(
            "Secure Share frame metadata {name} is empty"
        )));
    }
    Ok(())
}

fn validate_scale(value: f32, name: &str) -> Result<()> {
    if !value.is_finite() || value <= 0.0 {
        return Err(AppError::InvalidConfig(format!(
            "Secure Share frame metadata {name} is invalid"
        )));
    }
    Ok(())
}
