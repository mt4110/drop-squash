use std::sync::mpsc::{sync_channel, RecvTimeoutError};
use std::time::Duration;

use block2::RcBlock;
use dropsquash_core::{AppError, Result};
use objc2::rc::Retained;
use objc2_foundation::NSError;
use objc2_screen_capture_kit::SCStream;

pub type StreamCompletion = RcBlock<dyn Fn(*mut NSError) + 'static>;

pub fn start_stream_capture(stream: &SCStream, timeout: Duration) -> Result<()> {
    wait_for_stream_completion("start", timeout, |completion| unsafe {
        stream.startCaptureWithCompletionHandler(Some(&completion));
    })
}

pub fn stop_stream_capture(stream: &SCStream, timeout: Duration) -> Result<()> {
    wait_for_stream_completion("stop", timeout, |completion| unsafe {
        stream.stopCaptureWithCompletionHandler(Some(&completion));
    })
}

pub(crate) fn wait_for_stream_completion(
    action: &str,
    timeout: Duration,
    invoker: impl FnOnce(StreamCompletion),
) -> Result<()> {
    let (sender, receiver) = sync_channel(1);
    let completion_action = action.to_string();
    let completion = RcBlock::new(move |error| {
        let _ = sender.send(completion_result(&completion_action, error));
    });
    invoker(completion);
    match receiver.recv_timeout(timeout) {
        Ok(result) => result,
        Err(RecvTimeoutError::Timeout) => Err(fail_closed(action, "timed out")),
        Err(RecvTimeoutError::Disconnected) => Err(fail_closed(action, "disconnected")),
    }
}

fn completion_result(action: &str, error: *mut NSError) -> Result<()> {
    if error.is_null() {
        return Ok(());
    }
    Err(fail_closed(
        action,
        &format!("failed: {}", retained_error_message(error)),
    ))
}

fn retained_error_message(error: *mut NSError) -> String {
    unsafe { Retained::retain(error) }
        .map(|error| error.to_string())
        .unwrap_or_else(|| "unknown ScreenCaptureKit error".to_string())
}

fn fail_closed(action: &str, reason: &str) -> AppError {
    AppError::InvalidConfig(format!(
        "Secure Share ScreenCaptureKit stream {action} {reason}"
    ))
}
