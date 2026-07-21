use std::path::PathBuf;
use std::sync::mpsc::sync_channel;
use std::sync::Mutex;
use std::time::Duration;

use super::finalize::SecureShareRecordingDto;
use super::paths::{recording_paths, RecordingPaths};
use super::publication::PublicationPermit;
use super::selection::SecureShareWindowSelectionDto;
use dropsquash_core::OutputSize;
use state::{reserve, take, ActiveRecording};

mod native;
mod state;

#[derive(Default)]
pub struct SecureShareRecordingState {
    active: Mutex<Option<ActiveRecording>>,
}

#[tauri::command(rename_all = "camelCase")]
pub async fn secure_share_recording_start(
    state: tauri::State<'_, SecureShareRecordingState>,
    selection: SecureShareWindowSelectionDto,
    output_dir: String,
    output_size: OutputSize,
) -> Result<(), String> {
    let paths = recording_paths(PathBuf::from(output_dir))?;
    let (stop, stop_receiver) = sync_channel(1);
    let (ready, ready_receiver) = sync_channel(1);
    let (finished, finished_receiver) = sync_channel(1);
    let permit = PublicationPermit::default();
    reserve(
        &state,
        stop,
        finished_receiver,
        paths.clone(),
        permit.clone(),
    )?;
    native::start(
        selection,
        output_size,
        paths.clone(),
        permit,
        stop_receiver,
        ready,
        finished,
    );
    let ready = tauri::async_runtime::spawn_blocking(move || {
        ready_receiver.recv_timeout(Duration::from_secs(15))
    })
    .await
    .map_err(|error| format!("Secure Share recording start task failed: {error}"))?
    .map_err(|_| "Secure Share recording discovery timed out".to_string())?;
    if let Err(error) = ready {
        discard(&state, &paths);
        return Err(error);
    }
    Ok(())
}

#[tauri::command]
pub async fn secure_share_recording_stop(
    state: tauri::State<'_, SecureShareRecordingState>,
) -> Result<SecureShareRecordingDto, String> {
    let active = take(&state)?;
    let stop_sent = active.stop.send(()).is_ok();
    let permit = active.permit.clone();
    let paths = active.paths.clone();
    let result = tauri::async_runtime::spawn_blocking(move || {
        active.finished.recv_timeout(Duration::from_secs(30))
    })
    .await
    .map_err(|error| format!("Secure Share recording stop task failed: {error}"))?
    .map_err(|error| finalization_error(stop_sent, error))?;
    match result {
        Ok(result) if stop_sent => Ok(result),
        Ok(_) => {
            permit.cancel(&paths);
            Err("Secure Share recording ended before the stop request".to_string())
        }
        Err(error) => {
            permit.cancel(&paths);
            Err(error)
        }
    }
}

fn finalization_error(stop_sent: bool, error: std::sync::mpsc::RecvTimeoutError) -> String {
    let phase = if stop_sent {
        "Secure Share recording finalization"
    } else {
        "Secure Share recording ended before stop"
    };
    let reason = match error {
        std::sync::mpsc::RecvTimeoutError::Timeout => "timed out",
        std::sync::mpsc::RecvTimeoutError::Disconnected => "disconnected",
    };
    format!("{phase}: {reason}")
}

fn discard(state: &SecureShareRecordingState, paths: &RecordingPaths) {
    state::discard(state, paths);
}

pub fn cancel_active(state: &SecureShareRecordingState) {
    state::cancel_active(state);
}

#[cfg(test)]
#[path = "recording/tests.rs"]
mod tests;
