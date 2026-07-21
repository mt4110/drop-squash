use std::sync::mpsc::{Receiver, SyncSender, TryRecvError};
use std::time::Duration;

use dropsquash_core::CaptureFrameMetadata;
use dropsquash_platform::{
    capture_frame_metadata_from_native, NativeAccessibilityObservation, NativeRecordingEvent,
    NativeStrictRecordingHandle, NativeTemporalObservation, NativeVisionObservation,
};

mod accessibility;
mod frame_validation;
mod temporal;
mod terminal;
mod vision;
use accessibility::drain_accessibility;
use frame_validation::validate;
use temporal::drain as drain_temporal;
use terminal::reject_early;
use vision::drain as drain_vision;

type StoppedValues = (
    Vec<CaptureFrameMetadata>,
    Vec<NativeVisionObservation>,
    Vec<NativeAccessibilityObservation>,
    Vec<NativeTemporalObservation>,
);

pub(super) fn wait_started(
    events: &Receiver<NativeRecordingEvent>,
    ready: &SyncSender<Result<(), String>>,
) -> Result<(), String> {
    let result = match events.recv_timeout(Duration::from_secs(15)) {
        Ok(NativeRecordingEvent::Started) => ready
            .send(Ok(()))
            .map_err(|_| "Secure Share startup receiver disconnected".to_string()),
        Ok(NativeRecordingEvent::Failed { reason, .. }) => Err(reason.user_message().to_string()),
        Ok(NativeRecordingEvent::Completed { .. }) => {
            Err("Secure Share native recording ended before start".to_string())
        }
        Err(_) => Err("Secure Share native recording start timed out".to_string()),
    };
    if let Err(error) = &result {
        let _ = ready.send(Err(error.clone()));
    }
    result
}

pub(super) fn wait_stopped(
    handle: &NativeStrictRecordingHandle,
    stop: Receiver<()>,
) -> Result<StoppedValues, String> {
    let mut frames = Vec::new();
    let mut vision = Vec::new();
    let mut accessibility = Vec::new();
    let mut temporal = Vec::new();
    loop {
        drain(
            handle,
            &mut frames,
            &mut vision,
            &mut accessibility,
            &mut temporal,
        )?;
        reject_early(&handle.events)?;
        match stop.recv_timeout(Duration::from_millis(10)) {
            Ok(()) => break,
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                return Err("Secure Share stop control disconnected".to_string())
            }
        }
    }
    handle.stop();
    loop {
        drain(
            handle,
            &mut frames,
            &mut vision,
            &mut accessibility,
            &mut temporal,
        )?;
        match handle.events.recv_timeout(Duration::from_millis(10)) {
            Ok(NativeRecordingEvent::Completed { frame_count }) => {
                drain(
                    handle,
                    &mut frames,
                    &mut vision,
                    &mut accessibility,
                    &mut temporal,
                )?;
                if frame_count == frames.len() as u64 {
                    validate(&frames)?;
                    return Ok((frames, vision, accessibility, temporal));
                }
                return Err("Secure Share native frame count mismatch".to_string());
            }
            Ok(NativeRecordingEvent::Failed { reason, .. }) => {
                return Err(reason.user_message().to_string())
            }
            Ok(NativeRecordingEvent::Started) | Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
            }
            Err(_) => return Err("Secure Share native recording callback disconnected".to_string()),
        }
    }
}

fn drain(
    handle: &NativeStrictRecordingHandle,
    frames: &mut Vec<CaptureFrameMetadata>,
    vision: &mut Vec<NativeVisionObservation>,
    accessibility: &mut Vec<NativeAccessibilityObservation>,
    temporal: &mut Vec<NativeTemporalObservation>,
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
    drain_temporal(handle, temporal)
}
