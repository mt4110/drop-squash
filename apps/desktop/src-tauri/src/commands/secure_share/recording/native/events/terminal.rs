use std::sync::mpsc::{Receiver, TryRecvError};

use dropsquash_platform::NativeRecordingEvent;

pub(super) fn reject_early(events: &Receiver<NativeRecordingEvent>) -> Result<(), String> {
    match events.try_recv() {
        Ok(NativeRecordingEvent::Failed { reason, .. }) => Err(reason.user_message().to_string()),
        Ok(NativeRecordingEvent::Completed { .. }) => {
            Err("Secure Share native recording ended before stop".to_string())
        }
        Ok(NativeRecordingEvent::Started) | Err(TryRecvError::Empty) => Ok(()),
        Err(TryRecvError::Disconnected) => {
            Err("Secure Share native recording callback disconnected".to_string())
        }
    }
}
