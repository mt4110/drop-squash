use dropsquash_core::{
    AxObservation, CaptureFrameMetadata, FrameSize, MaskPlan, MaskPlanDraft, MaskPolicy, Result,
    VerificationExpectations, VisionObservation,
};

use super::{
    capture_snapshot_from_providers, EmptyAccessibilityObservationProvider,
    EmptyVisionObservationProvider, NativeFrameMetadataProvider,
};

#[derive(Debug, Clone, PartialEq)]
pub struct SecureShareObservationSnapshot {
    pub frame_size: FrameSize,
    pub frames: Vec<CaptureFrameMetadata>,
    pub accessibility: Vec<AxObservation>,
    pub vision: Vec<VisionObservation>,
}

impl SecureShareObservationSnapshot {
    pub fn into_mask_plan(
        self,
        capture_id: String,
        policy: MaskPolicy,
        verification_expectations: VerificationExpectations,
    ) -> MaskPlan {
        MaskPlanDraft {
            capture_id,
            frame_size: self.frame_size,
            frames: self.frames,
            accessibility: self.accessibility,
            vision: self.vision,
            temporal: Vec::new(),
            policy,
            verification_expectations,
        }
        .into_mask_plan()
    }
}

#[derive(Debug, Clone, Default)]
pub struct SecureShareProbe;

pub trait SecureShareSnapshotProvider {
    fn capture_snapshot(&self) -> Result<SecureShareObservationSnapshot>;
}

impl SecureShareProbe {
    pub fn capture_snapshot(&self) -> Result<SecureShareObservationSnapshot> {
        self.capture_snapshot_with(&NativeSecureShareSnapshotProvider)
    }

    pub fn capture_snapshot_with(
        &self,
        provider: &impl SecureShareSnapshotProvider,
    ) -> Result<SecureShareObservationSnapshot> {
        provider.capture_snapshot()
    }
}

#[derive(Debug, Clone, Default)]
struct NativeSecureShareSnapshotProvider;

impl SecureShareSnapshotProvider for NativeSecureShareSnapshotProvider {
    fn capture_snapshot(&self) -> Result<SecureShareObservationSnapshot> {
        capture_snapshot_from_providers(
            &NativeFrameMetadataProvider,
            &EmptyAccessibilityObservationProvider,
            &EmptyVisionObservationProvider,
        )
    }
}
