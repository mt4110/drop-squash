use std::sync::mpsc::{sync_channel, RecvTimeoutError};
use std::time::Duration;

use block2::RcBlock;
use dropsquash_core::{AppError, Result};
use objc2::rc::Retained;
use objc2_foundation::NSError;
use objc2_screen_capture_kit::SCShareableContent;

use super::shareable_content::{snapshot_shareable_content, SckShareableContentSnapshot};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SckShareableContentRequest {
    pub exclude_desktop_windows: bool,
    pub on_screen_windows_only: bool,
    pub timeout: Duration,
}

pub fn request_shareable_content_snapshot(
    request: SckShareableContentRequest,
) -> Result<SckShareableContentSnapshot> {
    request_shareable_content_snapshot_with(request, request_unredacted_shareable_content)
}

pub fn request_shareable_content_snapshot_with(
    request: SckShareableContentRequest,
    requester: impl FnOnce(SckShareableContentRequest, ShareableContentCompletion),
) -> Result<SckShareableContentSnapshot> {
    let (sender, receiver) = sync_channel(1);
    let completion = RcBlock::new(move |content, error| {
        let _ = sender.send(snapshot_from_completion(content, error));
    });
    requester(request, completion);
    match receiver.recv_timeout(request.timeout) {
        Ok(result) => result,
        Err(RecvTimeoutError::Timeout) => Err(fail_closed("request timed out")),
        Err(RecvTimeoutError::Disconnected) => Err(fail_closed("completion disconnected")),
    }
}

pub type ShareableContentCompletion =
    RcBlock<dyn Fn(*mut SCShareableContent, *mut NSError) + 'static>;

fn request_unredacted_shareable_content(
    request: SckShareableContentRequest,
    completion: ShareableContentCompletion,
) {
    unsafe {
        SCShareableContent::getShareableContentExcludingDesktopWindows_onScreenWindowsOnly_completionHandler(
            request.exclude_desktop_windows,
            request.on_screen_windows_only,
            &completion,
        );
    }
}

pub(crate) fn snapshot_from_completion(
    content: *mut SCShareableContent,
    error: *mut NSError,
) -> Result<SckShareableContentSnapshot> {
    if !error.is_null() {
        return Err(fail_closed(&format!(
            "request failed: {}",
            retained_error_message(error)
        )));
    }
    if content.is_null() {
        return Err(fail_closed("request returned no shareable content"));
    }
    let content = unsafe { Retained::retain(content) }
        .ok_or_else(|| fail_closed("request returned unrecoverable shareable content"))?;
    snapshot_shareable_content(&content)
}

pub(super) fn retained_error_message(error: *mut NSError) -> String {
    unsafe { Retained::retain(error) }
        .map(|error| error.to_string())
        .unwrap_or_else(|| "unknown ScreenCaptureKit error".to_string())
}

fn fail_closed(reason: &str) -> AppError {
    AppError::InvalidConfig(format!(
        "Secure Share ScreenCaptureKit shareable content {reason}"
    ))
}
