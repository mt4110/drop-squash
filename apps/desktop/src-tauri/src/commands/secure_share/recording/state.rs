use std::sync::mpsc::{Receiver, SyncSender};

use super::super::finalize::SecureShareRecordingDto;
use super::super::paths::RecordingPaths;
use super::super::publication::PublicationPermit;
use super::SecureShareRecordingState;

pub(super) struct ActiveRecording {
    pub(super) stop: SyncSender<()>,
    pub(super) finished: Receiver<Result<SecureShareRecordingDto, String>>,
    pub(super) paths: RecordingPaths,
    pub(super) permit: PublicationPermit,
}

#[cfg(test)]
mod tests;

pub(super) fn reserve(
    state: &SecureShareRecordingState,
    stop: SyncSender<()>,
    finished: Receiver<Result<SecureShareRecordingDto, String>>,
    paths: RecordingPaths,
    permit: PublicationPermit,
) -> Result<(), String> {
    let mut active = state
        .active
        .lock()
        .map_err(|_| "Secure Share recording state is unavailable".to_string())?;
    if active.is_some() {
        return Err("A Secure Share recording is already running.".to_string());
    }
    *active = Some(ActiveRecording {
        stop,
        finished,
        paths,
        permit,
    });
    Ok(())
}

pub(super) fn take(state: &SecureShareRecordingState) -> Result<ActiveRecording, String> {
    state
        .active
        .lock()
        .map_err(|_| "Secure Share recording state is unavailable".to_string())?
        .take()
        .ok_or_else(|| "No Secure Share recording is running.".to_string())
}

pub(super) fn discard(state: &SecureShareRecordingState, fallback: &RecordingPaths) {
    if cancel_active(state) {
        return;
    }
    let _ = std::fs::remove_file(&fallback.partial);
}

pub(super) fn cancel_active(state: &SecureShareRecordingState) -> bool {
    let Ok(mut active) = state.active.lock() else {
        return false;
    };
    let Some(active) = active.take() else {
        return false;
    };
    let _ = active.stop.send(());
    active.permit.cancel(&active.paths);
    true
}

impl Drop for SecureShareRecordingState {
    fn drop(&mut self) {
        let Ok(active) = self.active.get_mut() else {
            return;
        };
        if let Some(active) = active.take() {
            let _ = active.stop.send(());
            active.permit.cancel(&active.paths);
        }
    }
}
