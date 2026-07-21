pub type RecordingCallback = unsafe extern "C" fn(i32, i32, u64, u64, *mut std::ffi::c_void);
pub type FrameMetadataCallback =
    unsafe extern "C" fn(u64, *const NativeFrameMetadata, *mut std::ffi::c_void) -> i32;
pub type VisionObservationCallback =
    unsafe extern "C" fn(u64, *const NativeVisionObservation, *mut std::ffi::c_void) -> i32;
pub type AccessibilityObservationCallback =
    unsafe extern "C" fn(u64, *const NativeAccessibilityObservation, *mut std::ffi::c_void) -> i32;

mod temporal;
pub use temporal::{NativeTemporalObservation, TemporalObservationCallback};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NativeFrameMetadata {
    pub frame_index: u64,
    pub display_time_ticks: u64,
    pub frame_status: i32,
    pub scale_factor: f32,
    pub content_scale: f32,
    pub content_x: i32,
    pub content_y: i32,
    pub content_width: u32,
    pub content_height: u32,
    pub bounding_x: i32,
    pub bounding_y: i32,
    pub bounding_width: u32,
    pub bounding_height: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NativeVisionObservation {
    pub frame_index: u64,
    pub kind: i32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub confidence: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeAccessibilityObservation {
    pub frame_index: u64,
    pub kind: i32,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

unsafe extern "C" {
    fn dropsquash_secure_share_bridge_start_strict_recording(
        window_id: u32,
        width: u32,
        height: u32,
        path: *const std::ffi::c_char,
        context: *mut std::ffi::c_void,
        callback: RecordingCallback,
    ) -> u64;
    fn dropsquash_secure_share_bridge_start_strict_recording_with_metadata(
        window_id: u32,
        width: u32,
        height: u32,
        path: *const std::ffi::c_char,
        context: *mut std::ffi::c_void,
        callback: RecordingCallback,
        metadata: FrameMetadataCallback,
    ) -> u64;
    fn dropsquash_secure_share_bridge_stop_recording(session_id: u64);
}

/// # Safety
///
/// `path` must be valid for this call. `context` must outlive a terminal callback.
pub unsafe fn start_strict_recording(
    window_id: u32,
    width: u32,
    height: u32,
    path: *const std::ffi::c_char,
    context: *mut std::ffi::c_void,
    callback: RecordingCallback,
) -> u64 {
    unsafe {
        dropsquash_secure_share_bridge_start_strict_recording(
            window_id, width, height, path, context, callback,
        )
    }
}

/// # Safety
///
/// `context` must outlive both terminal and per-frame callbacks. The metadata
/// callback must return nonzero when it cannot retain a value-only frame event.
pub unsafe fn start_strict_recording_with_metadata(
    window_id: u32,
    width: u32,
    height: u32,
    path: *const std::ffi::c_char,
    context: *mut std::ffi::c_void,
    callback: RecordingCallback,
    metadata: FrameMetadataCallback,
) -> u64 {
    unsafe {
        dropsquash_secure_share_bridge_start_strict_recording_with_metadata(
            window_id, width, height, path, context, callback, metadata,
        )
    }
}

pub fn stop_strict_recording(session_id: u64) {
    unsafe { dropsquash_secure_share_bridge_stop_recording(session_id) };
}

mod attested;
pub use attested::{
    start_attested_strict_recording_with_observations, AttestedObservationCallbacks,
    AttestedRecordingRequest,
};
mod attested_temporal;
pub use attested_temporal::{
    start_attested_strict_recording_with_temporal_observations, AttestedTemporalCallbacks,
};
mod session;
pub use session::{
    start_attested_native_strict_recording, start_native_strict_recording, NativeRecordingEvent,
    NativeRecordingFailure, NativeStrictRecordingHandle,
};
