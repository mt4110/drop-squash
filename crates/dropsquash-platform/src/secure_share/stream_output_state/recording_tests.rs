use super::RecordingState;

#[test]
fn discarding_removes_a_partial_path_before_the_session_stops() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("recording.partial.mp4");
    std::fs::write(&path, b"partial").unwrap();
    let mut state = RecordingState::enabled(path.clone());

    state.discard();

    assert!(!path.exists());
    assert!(state.finish().unwrap().is_none());
}
