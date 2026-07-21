use std::sync::mpsc::sync_channel;
use std::time::Duration;

use super::*;
use tempfile::TempDir;

#[test]
fn drop_removes_partial_and_stops_recording() {
    let (_directory, state, paths, stop) = active_state();

    drop(state);

    assert!(stop.recv_timeout(Duration::ZERO).is_ok());
    assert!(!paths.partial.exists());
}

#[test]
fn cancel_active_removes_partial_and_stops_recording() {
    let (_directory, state, paths, stop) = active_state();

    assert!(cancel_active(&state));

    assert!(stop.recv_timeout(Duration::ZERO).is_ok());
    assert!(!paths.partial.exists());
}

#[test]
fn discard_stops_a_recording_left_by_failed_startup() {
    let (_directory, state, paths, stop) = active_state();

    discard(&state, &paths);

    assert!(stop.recv_timeout(Duration::ZERO).is_ok());
    assert!(!paths.partial.exists());
}

fn active_state() -> (
    TempDir,
    SecureShareRecordingState,
    RecordingPaths,
    std::sync::mpsc::Receiver<()>,
) {
    let directory = tempfile::tempdir().unwrap();
    let paths = RecordingPaths {
        partial: directory.path().join("recording.partial.mp4"),
        final_path: directory.path().join("recording.mp4"),
    };
    std::fs::write(&paths.partial, b"partial").unwrap();
    let (stop, stop_receiver) = sync_channel(1);
    let (_, finished) = sync_channel(1);
    let state = SecureShareRecordingState {
        active: std::sync::Mutex::new(Some(ActiveRecording {
            stop,
            finished,
            paths: paths.clone(),
            permit: PublicationPermit::default(),
        })),
    };
    (directory, state, paths, stop_receiver)
}
