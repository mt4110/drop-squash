use std::ffi::{c_void, CString};
use std::path::Path;
use std::sync::mpsc::sync_channel;
use std::sync::Arc;

use dropsquash_core::CaptureRect;

use super::super::{
    start_attested_strict_recording_with_temporal_observations,
    start_strict_recording_with_metadata,
};
use super::callbacks::{
    accessibility_callback, metadata_callback, recording_callback, temporal_callback,
    vision_callback,
};
use super::{CallbackContext, NativeStrictRecordingHandle};

const FRAME_EVENT_CAPACITY: usize = 120;
const ACCESSIBILITY_EVENT_CAPACITY: usize = 2_048;

pub fn start_native_strict_recording(
    window_id: u32,
    width: u32,
    height: u32,
    path: &Path,
) -> Result<NativeStrictRecordingHandle, String> {
    start(path, |path, context| unsafe {
        start_strict_recording_with_metadata(
            window_id,
            width,
            height,
            path,
            context,
            recording_callback,
            metadata_callback,
        )
    })
}

pub fn start_attested_native_strict_recording(
    window_id: u32,
    owner_pid: i32,
    frame: CaptureRect,
    width: u32,
    height: u32,
    path: &Path,
) -> Result<NativeStrictRecordingHandle, String> {
    start(path, |path, context| unsafe {
        start_attested_strict_recording_with_temporal_observations(
            window_id,
            owner_pid,
            frame,
            width,
            height,
            path,
            context,
            recording_callback,
            metadata_callback,
            vision_callback,
            accessibility_callback,
            temporal_callback,
        )
    })
}

fn start(
    output: &Path,
    native_start: impl FnOnce(*const i8, *mut c_void) -> u64,
) -> Result<NativeStrictRecordingHandle, String> {
    let path = CString::new(output.as_os_str().as_encoded_bytes())
        .map_err(|_| "Secure Share output path contains a NUL byte".to_string())?;
    let (events_sender, events) = sync_channel(2);
    let (metadata_sender, metadata) = sync_channel(FRAME_EVENT_CAPACITY);
    let (vision_sender, vision) = sync_channel(FRAME_EVENT_CAPACITY);
    let (accessibility_sender, accessibility) = sync_channel(ACCESSIBILITY_EVENT_CAPACITY);
    let (temporal_sender, temporal) = sync_channel(FRAME_EVENT_CAPACITY);
    let context = Arc::new(CallbackContext {
        events: events_sender,
        metadata: metadata_sender,
        vision: vision_sender,
        accessibility: accessibility_sender,
        temporal: temporal_sender,
    });
    let native_context = Arc::into_raw(Arc::clone(&context))
        .cast_mut()
        .cast::<c_void>();
    let session_id = native_start(path.as_ptr(), native_context);
    if session_id == 0 {
        unsafe { drop(Arc::from_raw(native_context.cast::<CallbackContext>())) };
        return Err("Secure Share native recording could not start".to_string());
    }
    Ok(NativeStrictRecordingHandle {
        session_id,
        events,
        metadata,
        vision,
        accessibility,
        temporal,
        _context: context,
    })
}
