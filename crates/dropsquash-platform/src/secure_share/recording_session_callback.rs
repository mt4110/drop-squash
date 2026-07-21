use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

use block2::RcBlock;
use dropsquash_core::{MaskPolicy, OutputSize, Result};
use objc2::rc::Retained;
use objc2_foundation::NSError;
use objc2_screen_capture_kit::SCShareableContent;

use super::{SckObservationProbeRequest, SckRecordingProbeReport, SckWindowSelection};

type Completed = Arc<dyn Fn(Result<SckRecordingProbeReport>) + Send + Sync>;
type Started = Arc<Mutex<Option<Box<dyn FnOnce(std::result::Result<(), String>) + Send>>>>;
type Stop = Arc<Mutex<Option<Receiver<()>>>>;

pub struct RecordingCallbackRequest {
    pub selection: SckWindowSelection,
    pub output_path: PathBuf,
    pub output_size: OutputSize,
    pub policy: MaskPolicy,
    pub probe: SckObservationProbeRequest,
    pub stop: Receiver<()>,
}

pub fn record_window_until_stopped_callback(
    request: RecordingCallbackRequest,
    started: impl FnOnce(std::result::Result<(), String>) + Send + 'static,
    completed: impl Fn(Result<SckRecordingProbeReport>) + Send + Sync + 'static,
) {
    let completed: Completed = Arc::new(completed);
    let started: Started = Arc::new(Mutex::new(Some(Box::new(started))));
    let stop: Stop = Arc::new(Mutex::new(Some(request.stop)));
    let completion = Box::leak(Box::new(RcBlock::new(
        move |content: *mut SCShareableContent, error: *mut NSError| {
            if !error.is_null() {
                notify_started(
                    &started,
                    Err("Secure Share recording discovery failed".to_string()),
                );
                completed(Err(super::recording_probe::fail_closed("discovery failed")));
                return;
            }
            let Some(content) = (unsafe { Retained::retain(content) }) else {
                notify_started(
                    &started,
                    Err("Secure Share recording discovery returned no content".to_string()),
                );
                completed(Err(super::recording_probe::fail_closed(
                    "discovery returned no content",
                )));
                return;
            };
            let Some(stop) = take_stop(&stop) else {
                notify_started(
                    &started,
                    Err("Secure Share recording was already started".to_string()),
                );
                return;
            };
            let content_addr = Retained::into_raw(content) as usize;
            let started_for_recording = Arc::clone(&started);
            let completed_for_recording = Arc::clone(&completed);
            let output_path = request.output_path.clone();
            let output_size = request.output_size;
            let policy = request.policy;
            let probe = request.probe;
            let selection = request.selection.clone();
            std::thread::spawn(move || {
                let content =
                    unsafe { Retained::from_raw(content_addr as *mut SCShareableContent) }
                        .expect("retained ScreenCaptureKit content must not be null");
                completed_for_recording(
                    super::recording_session::record_until_stopped_from_content(
                        super::recording_session::request::RecordingSessionRequest {
                            selection,
                            output_path,
                            output_size,
                            policy,
                            probe,
                            content,
                            stop,
                        },
                        move |result| notify_started(&started_for_recording, result),
                    ),
                );
            });
        },
    )));
    unsafe {
        SCShareableContent::getShareableContentExcludingDesktopWindows_onScreenWindowsOnly_completionHandler(
            true,
            true,
            completion,
        );
    }
}

fn take_stop(stop: &Stop) -> Option<Receiver<()>> {
    stop.lock().ok()?.take()
}

fn notify_started(started: &Started, result: std::result::Result<(), String>) {
    if let Ok(mut callback) = started.lock() {
        if let Some(callback) = callback.take() {
            callback(result);
        }
    }
}
