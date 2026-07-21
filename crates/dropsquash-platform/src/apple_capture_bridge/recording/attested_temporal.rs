use std::ffi::{c_char, c_void};

use super::attested::AttestedRecordingRequest;
use super::{
    AccessibilityObservationCallback, FrameMetadataCallback, RecordingCallback,
    TemporalObservationCallback, VisionObservationCallback,
};

pub struct AttestedTemporalCallbacks {
    pub recording: RecordingCallback,
    pub metadata: FrameMetadataCallback,
    pub vision: VisionObservationCallback,
    pub accessibility: AccessibilityObservationCallback,
    pub temporal: TemporalObservationCallback,
}

unsafe extern "C" {
    fn dropsquash_secure_share_bridge_start_attested_strict_recording_with_temporal_observations(
        window_id: u32,
        owner_pid: i32,
        x: i32,
        y: i32,
        window_width: u32,
        window_height: u32,
        output_width: u32,
        output_height: u32,
        path: *const c_char,
        context: *mut c_void,
        callback: RecordingCallback,
        metadata: FrameMetadataCallback,
        vision: VisionObservationCallback,
        accessibility: AccessibilityObservationCallback,
        temporal: TemporalObservationCallback,
    ) -> u64;
}

/// # Safety
///
/// `context` must outlive every value-only callback and terminal callback.
pub unsafe fn start_attested_strict_recording_with_temporal_observations(
    request: AttestedRecordingRequest,
    callbacks: AttestedTemporalCallbacks,
) -> u64 {
    unsafe {
        dropsquash_secure_share_bridge_start_attested_strict_recording_with_temporal_observations(
            request.window_id,
            request.owner_pid,
            request.frame.x,
            request.frame.y,
            request.frame.width,
            request.frame.height,
            request.output_width,
            request.output_height,
            request.path,
            request.context,
            callbacks.recording,
            callbacks.metadata,
            callbacks.vision,
            callbacks.accessibility,
            callbacks.temporal,
        )
    }
}
