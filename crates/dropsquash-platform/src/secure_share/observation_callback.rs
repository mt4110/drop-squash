use std::sync::Arc;

use block2::RcBlock;
use dropsquash_core::Result;
use objc2::rc::Retained;
use objc2_foundation::NSError;
use objc2_screen_capture_kit::SCShareableContent;

use super::{
    observation_probe::observe_from_content, select_explicit_window_target,
    SckObservationProbeReport, SckObservationProbeRequest,
};

type ObserveCallback = Arc<dyn Fn(Result<SckObservationProbeReport>) + Send + Sync>;

pub fn observe_window_once_callback(
    window_id: u32,
    request: SckObservationProbeRequest,
    callback: impl Fn(Result<SckObservationProbeReport>) + Send + Sync + 'static,
) {
    let callback: ObserveCallback = Arc::new(callback);
    let completion = Box::leak(Box::new(RcBlock::new(move |content, error| {
        handle_discovery(window_id, request, Arc::clone(&callback), content, error);
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
    request: SckObservationProbeRequest,
    callback: ObserveCallback,
    content: *mut SCShareableContent,
    error: *mut NSError,
) {
    let content = match retained_content(content, error) {
        Ok(content) => content,
        Err(reason) => {
            callback(Err(super::observation_probe::fail_closed(&reason)));
            return;
        }
    };
    let Some(content) = content else {
        callback(Err(super::observation_probe::fail_closed(
            "discovery returned no content",
        )));
        return;
    };
    let content_addr = Retained::into_raw(content) as usize;
    std::thread::spawn(move || {
        let content = unsafe { Retained::from_raw(content_addr as *mut SCShareableContent) }
            .expect("retained ScreenCaptureKit content must not be null");
        let result = observe_from_content(content, request, &|snapshot| {
            select_explicit_window_target(snapshot, window_id)
        });
        callback(result);
    });
}

fn retained_content(
    content: *mut SCShareableContent,
    error: *mut NSError,
) -> std::result::Result<Option<Retained<SCShareableContent>>, String> {
    if !error.is_null() {
        return Err(format!(
            "discovery failed: {}",
            retained_error_message(error)
        ));
    }
    Ok(unsafe { Retained::retain(content) })
}

fn retained_error_message(error: *mut NSError) -> String {
    unsafe { Retained::retain(error) }
        .map(|error| error.to_string())
        .unwrap_or_else(|| "unknown ScreenCaptureKit error".to_string())
}
