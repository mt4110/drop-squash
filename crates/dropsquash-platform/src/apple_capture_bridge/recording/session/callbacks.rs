use std::ffi::c_void;
use std::sync::Arc;

use super::{
    CallbackContext, NativeAccessibilityObservation, NativeDestructionEvidence,
    NativeFrameMetadata, NativeRecordingEvent, NativeTemporalObservation, NativeVisionObservation,
};

pub(super) unsafe extern "C" fn recording_callback(
    status: i32,
    reason: i32,
    _session_id: u64,
    frame_count: u64,
    context: *mut c_void,
) {
    let Some(context) = (unsafe { callback_context(context) }) else {
        return;
    };
    let terminal = match status {
        1 => context.events.try_send(NativeRecordingEvent::Started),
        2 => context
            .events
            .try_send(NativeRecordingEvent::Completed { frame_count }),
        _ => context.events.try_send(NativeRecordingEvent::Failed {
            frame_count,
            reason: reason.into(),
        }),
    };
    let _ = terminal;
    if status != 1 {
        unsafe { drop(Arc::from_raw(context as *const CallbackContext)) };
    }
}

pub(super) unsafe extern "C" fn metadata_callback(
    _session_id: u64,
    value: *const NativeFrameMetadata,
    context: *mut c_void,
) -> i32 {
    callback_value(context, value, |context, value| {
        context.metadata.try_send(value)
    })
}

pub(super) unsafe extern "C" fn vision_callback(
    _session_id: u64,
    value: *const NativeVisionObservation,
    context: *mut c_void,
) -> i32 {
    callback_value(context, value, |context, value| {
        context.vision.try_send(value)
    })
}

pub(super) unsafe extern "C" fn accessibility_callback(
    _session_id: u64,
    value: *const NativeAccessibilityObservation,
    context: *mut c_void,
) -> i32 {
    callback_value(context, value, |context, value| {
        context.accessibility.try_send(value)
    })
}

pub(super) unsafe extern "C" fn temporal_callback(
    _session_id: u64,
    value: *const NativeTemporalObservation,
    context: *mut c_void,
) -> i32 {
    callback_value(context, value, |context, value| {
        context.temporal.try_send(value)
    })
}

pub(super) unsafe extern "C" fn destruction_callback(
    _session_id: u64,
    value: *const NativeDestructionEvidence,
    context: *mut c_void,
) -> i32 {
    callback_value(context, value, |context, value| {
        context.destruction.try_send(value)
    })
}

fn callback_value<T: Copy>(
    context: *mut c_void,
    value: *const T,
    send: impl FnOnce(&CallbackContext, T) -> Result<(), std::sync::mpsc::TrySendError<T>>,
) -> i32 {
    let (Some(context), Some(value)) = (unsafe { callback_context(context) }, unsafe {
        value.as_ref()
    }) else {
        return 1;
    };
    i32::from(send(context, *value).is_err())
}

unsafe fn callback_context(pointer: *mut c_void) -> Option<&'static CallbackContext> {
    unsafe { pointer.cast::<CallbackContext>().as_ref() }
}
