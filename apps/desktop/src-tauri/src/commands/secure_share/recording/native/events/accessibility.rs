use std::sync::mpsc::TryRecvError;

use dropsquash_platform::{NativeAccessibilityObservation, NativeStrictRecordingHandle};

pub(super) fn drain_accessibility(
    handle: &NativeStrictRecordingHandle,
    accessibility: &mut Vec<NativeAccessibilityObservation>,
) -> Result<(), String> {
    loop {
        match handle.accessibility.try_recv() {
            Ok(value) => accessibility.push(value),
            Err(TryRecvError::Empty) => return Ok(()),
            Err(TryRecvError::Disconnected) => {
                return Err("Secure Share native Accessibility callback disconnected".to_string())
            }
        }
    }
}
