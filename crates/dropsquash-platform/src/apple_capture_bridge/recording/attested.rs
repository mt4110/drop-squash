use std::ffi::{c_char, c_void};

use dropsquash_core::CaptureRect;

use super::{FrameMetadataCallback, RecordingCallback, VisionObservationCallback};

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
    window_id: u32,
    owner_pid: i32,
    frame: CaptureRect,
    output_width: u32,
    output_height: u32,
    path: *const c_char,
    context: *mut c_void,
    callback: RecordingCallback,
    metadata: FrameMetadataCallback,
    vision: VisionObservationCallback,
) -> u64 {
    unsafe {
        dropsquash_secure_share_bridge_start_attested_strict_recording_with_observations(
            window_id,
            owner_pid,
            frame.x,
            frame.y,
            frame.width,
            frame.height,
            output_width,
            output_height,
            path,
            context,
            callback,
            metadata,
            vision,
        )
    }
}
