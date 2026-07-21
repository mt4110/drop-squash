use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::Arc;

use super::{
    stop_strict_recording, NativeAccessibilityObservation, NativeFrameMetadata,
    NativeTemporalObservation, NativeVisionObservation,
};

mod callbacks;
mod failure;
pub use failure::NativeRecordingFailure;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeRecordingEvent {
    Started,
    Completed {
        frame_count: u64,
    },
    Failed {
        frame_count: u64,
        reason: NativeRecordingFailure,
    },
}

pub struct NativeStrictRecordingHandle {
    pub session_id: u64,
    pub events: Receiver<NativeRecordingEvent>,
    pub metadata: Receiver<NativeFrameMetadata>,
    pub vision: Receiver<NativeVisionObservation>,
    pub accessibility: Receiver<NativeAccessibilityObservation>,
    pub temporal: Receiver<NativeTemporalObservation>,
    _context: Arc<CallbackContext>,
}

impl NativeStrictRecordingHandle {
    pub fn stop(&self) {
        stop_strict_recording(self.session_id);
    }
}

impl Drop for NativeStrictRecordingHandle {
    fn drop(&mut self) {
        self.stop();
    }
}

pub(super) struct CallbackContext {
    pub(super) events: SyncSender<NativeRecordingEvent>,
    pub(super) metadata: SyncSender<NativeFrameMetadata>,
    pub(super) vision: SyncSender<NativeVisionObservation>,
    pub(super) accessibility: SyncSender<NativeAccessibilityObservation>,
    pub(super) temporal: SyncSender<NativeTemporalObservation>,
}

mod start;
pub use start::{start_attested_native_strict_recording, start_native_strict_recording};

#[cfg(test)]
#[path = "session/tests.rs"]
mod tests;
