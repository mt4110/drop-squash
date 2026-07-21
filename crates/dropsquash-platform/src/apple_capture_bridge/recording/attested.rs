use std::ffi::{c_char, c_void};

use dropsquash_core::CaptureRect;

use super::{FrameMetadataCallback, RecordingCallback, VisionObservationCallback};

pub struct AttestedRecordingRequest {
    pub window_id: u32,
    pub owner_pid: i32,
    pub frame: CaptureRect,
    pub output_width: u32,
    pub output_height: u32,
    pub path: *const c_char,
    pub context: *mut c_void,
}

pub struct AttestedObservationCallbacks {
    pub recording: RecordingCallback,
    pub metadata: FrameMetadataCallback,
    pub vision: VisionObservationCallback,
}

unsafe extern "C" {
    fn dropsquash_secure_share_bridge_start_attested_strict_recording_with_observations(
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
    ) -> u64;
}

/// # Safety
///
/// `context` must outlive both terminal and per-frame callbacks.
pub unsafe fn start_attested_strict_recording_with_observations(
    request: AttestedRecordingRequest,
    callbacks: AttestedObservationCallbacks,
) -> u64 {
    unsafe {
        dropsquash_secure_share_bridge_start_attested_strict_recording_with_observations(
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
        )
    }
}
