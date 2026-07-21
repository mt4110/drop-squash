use std::sync::mpsc::TryRecvError;

use dropsquash_platform::{NativeDestructionEvidence, NativeStrictRecordingHandle};

pub(super) fn drain(
    handle: &NativeStrictRecordingHandle,
    evidence: &mut Vec<NativeDestructionEvidence>,
) -> Result<(), String> {
    loop {
        match handle.destruction.try_recv() {
            Ok(value) => evidence.push(value),
            Err(TryRecvError::Empty) => return Ok(()),
            Err(TryRecvError::Disconnected) => {
                return Err("Secure Share native destruction callback disconnected".to_string())
            }
        }
    }
}
