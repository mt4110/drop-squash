use std::sync::mpsc::TryRecvError;

use dropsquash_core::CaptureFrameMetadata;
use dropsquash_platform::{
    capture_frame_metadata_from_native, NativeAccessibilityObservation, NativeDestructionEvidence,
    NativeStrictRecordingHandle, NativeTemporalObservation, NativeVisionObservation,
};

use super::{drain_accessibility, drain_destruction, drain_temporal, drain_vision};

pub(super) fn values(
    handle: &NativeStrictRecordingHandle,
    frames: &mut Vec<CaptureFrameMetadata>,
    vision: &mut Vec<NativeVisionObservation>,
    accessibility: &mut Vec<NativeAccessibilityObservation>,
    temporal: &mut Vec<NativeTemporalObservation>,
    destruction: &mut Vec<NativeDestructionEvidence>,
) -> Result<(), String> {
    loop {
        match handle.metadata.try_recv() {
            Ok(value) => frames.push(
                capture_frame_metadata_from_native(value).map_err(|error| error.user_message())?,
            ),
            Err(TryRecvError::Empty) => break,
            Err(TryRecvError::Disconnected) => {
                return Err("Secure Share native metadata callback disconnected".to_string())
            }
        }
    }
    drain_vision(handle, vision)?;
    drain_accessibility(handle, accessibility)?;
    drain_temporal(handle, temporal)?;
    drain_destruction(handle, destruction)
}
