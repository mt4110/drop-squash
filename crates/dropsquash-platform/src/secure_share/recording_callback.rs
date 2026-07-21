use std::path::PathBuf;
use std::sync::Arc;

use block2::RcBlock;
use dropsquash_core::{MaskPolicy, Result};
use objc2::rc::Retained;
use objc2_foundation::NSError;
use objc2_screen_capture_kit::SCShareableContent;

use super::{SckObservationProbeRequest, SckRecordingProbeReport};

type RecordingCallback = Arc<dyn Fn(Result<SckRecordingProbeReport>) + Send + Sync>;

pub fn record_window_once_callback(
    window_id: u32,
    output_path: PathBuf,
    request: SckObservationProbeRequest,
    callback: impl Fn(Result<SckRecordingProbeReport>) + Send + Sync + 'static,
) {
    record_window_once_callback_with_policy(
        window_id,
        output_path,
        request,
        MaskPolicy::StrictReveal,
        callback,
    );
}

pub fn record_window_once_callback_with_policy(
    window_id: u32,
    output_path: PathBuf,
    request: SckObservationProbeRequest,
    policy: MaskPolicy,
    callback: impl Fn(Result<SckRecordingProbeReport>) + Send + Sync + 'static,
) {
    let callback: RecordingCallback = Arc::new(callback);
    let completion = Box::leak(Box::new(RcBlock::new(move |content, error| {
        handle_discovery(
            window_id,
            output_path.clone(),
            request,
            policy,
            Arc::clone(&callback),
            content,
            error,
        );
    })));
    unsafe {
        SCShareableContent::getShareableContentExcludingDesktopWindows_onScreenWindowsOnly_completionHandler(
            true,
            true,
            completion,
        );
    }
}

fn handle_discovery(
    window_id: u32,
    output_path: PathBuf,
    request: SckObservationProbeRequest,
    policy: MaskPolicy,
    callback: RecordingCallback,
    content: *mut SCShareableContent,
    error: *mut NSError,
) {
    if !error.is_null() {
        callback(Err(fail_closed("discovery failed")));
        return;
    }
    let Some(content) = (unsafe { Retained::retain(content) }) else {
        callback(Err(fail_closed("discovery returned no content")));
        return;
    };
    let content_addr = Retained::into_raw(content) as usize;
    std::thread::spawn(move || {
        let content = unsafe { Retained::from_raw(content_addr as *mut SCShareableContent) }
            .expect("retained ScreenCaptureKit content must not be null");
        callback(super::recording_probe::record_from_content_with_policy(
            window_id,
            output_path,
            request,
            policy,
            content,
        ));
    });
}

fn fail_closed(reason: &str) -> dropsquash_core::AppError {
    dropsquash_core::AppError::InvalidConfig(format!("Secure Share recording {reason}"))
}
