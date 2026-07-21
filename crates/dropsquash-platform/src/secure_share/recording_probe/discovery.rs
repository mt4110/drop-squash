use std::path::PathBuf;
use std::sync::mpsc::{sync_channel, RecvTimeoutError};

use block2::RcBlock;
use dropsquash_core::Result;
use objc2::rc::Retained;
use objc2_foundation::NSError;
use objc2_screen_capture_kit::SCShareableContent;

use super::{
    fail_closed, record_from_content, SckObservationProbeRequest, SckRecordingProbeReport,
};

pub fn record_window_once(
    window_id: u32,
    output_path: PathBuf,
    request: SckObservationProbeRequest,
) -> Result<SckRecordingProbeReport> {
    let (sender, receiver) = sync_channel(1);
    let completion = RcBlock::new(move |content, error| {
        let _ = sender.send(record_from_completion(
            window_id,
            output_path.clone(),
            request,
            content,
            error,
        ));
    });
    unsafe {
        SCShareableContent::getShareableContentExcludingDesktopWindows_onScreenWindowsOnly_completionHandler(
            true,
            true,
            &completion,
        );
    }
    receiver
        .recv_timeout(request.discovery_timeout)
        .map_err(wait_error)?
}

fn record_from_completion(
    window_id: u32,
    output_path: PathBuf,
    request: SckObservationProbeRequest,
    content: *mut SCShareableContent,
    error: *mut NSError,
) -> Result<SckRecordingProbeReport> {
    if !error.is_null() {
        return Err(fail_closed("discovery failed"));
    }
    let content = unsafe { Retained::retain(content) }.ok_or_else(|| fail_closed("no content"))?;
    record_from_content(window_id, output_path, request, content)
}

fn wait_error(error: RecvTimeoutError) -> dropsquash_core::AppError {
    match error {
        RecvTimeoutError::Timeout => fail_closed("discovery timed out"),
        RecvTimeoutError::Disconnected => fail_closed("discovery disconnected"),
    }
}
