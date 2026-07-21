use std::sync::mpsc::TryRecvError;

use dropsquash_platform::{NativeStrictRecordingHandle, NativeVisionObservation};

pub(super) fn drain(
    handle: &NativeStrictRecordingHandle,
    vision: &mut Vec<NativeVisionObservation>,
) -> Result<(), String> {
    loop {
        match handle.vision.try_recv() {
            Ok(value) => vision.push(value),
            Err(TryRecvError::Empty) => return Ok(()),
            Err(TryRecvError::Disconnected) => {
                return Err("Secure Share native Vision callback disconnected".to_string())
            }
        }
    }
}
