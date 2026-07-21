use std::sync::mpsc::{Receiver, SyncSender};
use std::time::Duration;

use dropsquash_platform::{NativeRecordingEvent, NativeStrictRecordingHandle};

mod accessibility;
mod destruction;
mod drain;
mod frame_validation;
mod temporal;
mod terminal;
mod values;
mod vision;
use accessibility::drain_accessibility;
use destruction::drain as drain_destruction;
use drain::values as drain;
use frame_validation::validate;
use temporal::drain as drain_temporal;
use terminal::reject_early;
use values::StoppedValues;
use vision::drain as drain_vision;

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
    let mut destruction = Vec::new();
    loop {
        drain(
            handle,
            &mut frames,
            &mut vision,
            &mut accessibility,
            &mut temporal,
            &mut destruction,
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
            &mut destruction,
        )?;
        match handle.events.recv_timeout(Duration::from_millis(10)) {
            Ok(NativeRecordingEvent::Completed { frame_count }) => {
                drain(
                    handle,
                    &mut frames,
                    &mut vision,
                    &mut accessibility,
                    &mut temporal,
                    &mut destruction,
                )?;
                if frame_count == frames.len() as u64 {
                    validate(&frames)?;
                    if destruction.len() == frames.len() {
                        return Ok((frames, vision, accessibility, temporal, destruction));
                    }
                    return Err("Secure Share native destruction evidence mismatch".to_string());
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
