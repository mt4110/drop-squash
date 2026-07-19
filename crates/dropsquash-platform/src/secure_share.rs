#[cfg(target_os = "macos")]
mod attachment_rect;
#[cfg(target_os = "macos")]
mod attachment_value;
mod attachments;
#[cfg(target_os = "macos")]
mod ax_observation;
#[cfg(target_os = "macos")]
mod ax_permission;
#[cfg(target_os = "macos")]
mod ax_probe;
#[cfg(target_os = "macos")]
mod ax_rect;
mod exports;
mod frame_info;
#[cfg(target_os = "macos")]
mod live_mask;
mod native_frame;
#[cfg(target_os = "macos")]
mod native_vision;
#[cfg(target_os = "macos")]
mod native_vision_observe;
#[cfg(target_os = "macos")]
mod native_vision_request;
#[cfg(target_os = "macos")]
mod native_vision_result;
#[cfg(target_os = "macos")]
mod observation_callback;
#[cfg(target_os = "macos")]
mod observation_probe;
mod providers;
#[cfg(target_os = "macos")]
mod sample_attachments;
#[cfg(target_os = "macos")]
mod sample_buffer;
#[cfg(target_os = "macos")]
mod sample_buffer_provider;
#[cfg(target_os = "macos")]
mod screen_permission;
#[cfg(target_os = "macos")]
mod shareable_content;
#[cfg(target_os = "macos")]
mod shareable_request;
#[cfg(target_os = "macos")]
mod stream_config;
#[cfg(target_os = "macos")]
mod stream_lifecycle;
#[cfg(target_os = "macos")]
mod stream_output;
#[cfg(target_os = "macos")]
mod stream_output_state;
#[cfg(target_os = "macos")]
mod stream_plan;
#[cfg(target_os = "macos")]
mod stream_registration;
#[cfg(target_os = "macos")]
mod target_policy;

pub use exports::*;

use dropsquash_core::{
    AxObservation, CaptureFrameMetadata, FrameSize, MaskPlan, MaskPlanDraft, MaskPolicy, Result,
    VerificationExpectations, VisionObservation,
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

#[cfg(test)]
mod tests;
