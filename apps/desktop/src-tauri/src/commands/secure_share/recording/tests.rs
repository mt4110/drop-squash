use std::sync::mpsc::RecvTimeoutError;

use super::finalization_error;

#[test]
fn reports_that_native_recording_ended_before_stop() {
    let message = finalization_error(false, RecvTimeoutError::Disconnected);

    assert!(message.contains("ended before stop"));
    assert!(message.contains("disconnected"));
}
