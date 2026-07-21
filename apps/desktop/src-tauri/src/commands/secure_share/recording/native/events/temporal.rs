use std::sync::mpsc::TryRecvError;

use dropsquash_platform::{NativeStrictRecordingHandle, NativeTemporalObservation};

pub(super) fn drain(
    handle: &NativeStrictRecordingHandle,
    temporal: &mut Vec<NativeTemporalObservation>,
) -> Result<(), String> {
    loop {
        match handle.temporal.try_recv() {
            Ok(value) => temporal.push(value),
            Err(TryRecvError::Empty) => return Ok(()),
            Err(TryRecvError::Disconnected) => {
                return Err("Secure Share native temporal callback disconnected".to_string())
            }
        }
    }
}
