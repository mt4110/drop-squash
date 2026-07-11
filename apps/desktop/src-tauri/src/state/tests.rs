use super::AppState;

#[test]
fn starts_and_finishes_conversion() {
    let state = AppState::default();

    assert!(state.start_conversion().is_ok());
    assert!(state.start_conversion().is_err());
    state.finish_conversion();
    assert!(state.start_conversion().is_ok());
}

#[test]
fn cancel_marks_active_token() {
    let state = AppState::default();
    let token = state.start_conversion().unwrap();

    assert!(state.cancel_conversion().unwrap());
    assert!(token.is_cancelled());
}

#[test]
fn cancel_without_active_conversion_reports_false() {
    let state = AppState::default();

    assert!(!state.cancel_conversion().unwrap());
}
