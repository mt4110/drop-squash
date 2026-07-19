use dropsquash_core::{AxObservation, CaptureFrameMetadata, FrameSize, Result, VisionObservation};

use super::SecureShareObservationSnapshot;

pub trait FrameMetadataProvider {
    fn frame_size(&self) -> Result<FrameSize>;
    fn frames(&self) -> Result<Vec<CaptureFrameMetadata>>;
}

pub trait AccessibilityObservationProvider {
    fn accessibility_observations(&self) -> Result<Vec<AxObservation>>;
}

pub trait VisionObservationProvider {
    fn vision_observations(&self) -> Result<Vec<VisionObservation>>;
}

#[derive(Debug, Clone, Default)]
pub struct EmptyAccessibilityObservationProvider;

impl AccessibilityObservationProvider for EmptyAccessibilityObservationProvider {
    fn accessibility_observations(&self) -> Result<Vec<AxObservation>> {
        Ok(Vec::new())
    }
}

#[derive(Debug, Clone, Default)]
pub struct EmptyVisionObservationProvider;

impl VisionObservationProvider for EmptyVisionObservationProvider {
    fn vision_observations(&self) -> Result<Vec<VisionObservation>> {
        Ok(Vec::new())
    }
}

pub fn capture_snapshot_from_providers(
    frame_provider: &impl FrameMetadataProvider,
    accessibility_provider: &impl AccessibilityObservationProvider,
    vision_provider: &impl VisionObservationProvider,
) -> Result<SecureShareObservationSnapshot> {
    Ok(SecureShareObservationSnapshot {
        frame_size: frame_provider.frame_size()?,
        frames: frame_provider.frames()?,
        accessibility: accessibility_provider.accessibility_observations()?,
        vision: vision_provider.vision_observations()?,
    })
}
