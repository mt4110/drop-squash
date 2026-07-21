use std::sync::mpsc::{sync_channel, RecvTimeoutError};
use std::time::Duration;

use block2::RcBlock;
use dropsquash_core::{
    AppError, AxObservation, CaptureFrameMetadata, FrameSize, Result, VisionObservation,
};
use objc2::rc::Retained;
use objc2_foundation::NSError;
use objc2_screen_capture_kit::SCShareableContent;

use super::{
    build_stream_capture_plan, select_explicit_window_target, select_strict_reveal_window_target,
    snapshot_shareable_content, SckCaptureTarget, SckFrameMetadataStreamRegistration,
    SckLiveMaskEvidence, SckShareableContentSnapshot,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SckObservationProbeRequest {
    pub discovery_timeout: Duration,
    pub completion_timeout: Duration,
    pub capture_duration: Duration,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SckObservationProbeReport {
    pub target: SckCaptureTarget,
    pub frame_size: FrameSize,
    pub frames: Vec<CaptureFrameMetadata>,
    pub accessibility: Vec<AxObservation>,
    pub vision: Vec<VisionObservation>,
    pub live_mask_evidence: SckLiveMaskEvidence,
}

pub fn observe_strict_reveal_window_once(
    request: SckObservationProbeRequest,
) -> Result<SckObservationProbeReport> {
    observe_with_selector(request, select_strict_reveal_window_target)
}

pub fn observe_window_once(
    window_id: u32,
    request: SckObservationProbeRequest,
) -> Result<SckObservationProbeReport> {
    observe_with_selector(request, move |snapshot| {
        select_explicit_window_target(snapshot, window_id)
    })
}

fn observe_with_selector(
    request: SckObservationProbeRequest,
    selector: impl Fn(&SckShareableContentSnapshot) -> Result<SckCaptureTarget> + 'static,
) -> Result<SckObservationProbeReport> {
    let (sender, receiver) = sync_channel(1);
    let completion = RcBlock::new(move |content, error| {
        let _ = sender.send(observe_from_completion(content, error, request, &selector));
    });
    unsafe {
        SCShareableContent::getShareableContentExcludingDesktopWindows_onScreenWindowsOnly_completionHandler(
            true,
            true,
            &completion,
        );
    }
    match receiver.recv_timeout(request.discovery_timeout) {
        Ok(result) => result,
        Err(RecvTimeoutError::Timeout) => Err(fail_closed("discovery timed out")),
        Err(RecvTimeoutError::Disconnected) => Err(fail_closed("discovery disconnected")),
    }
}

fn observe_from_completion(
    content: *mut SCShareableContent,
    error: *mut NSError,
    request: SckObservationProbeRequest,
    selector: &impl Fn(&SckShareableContentSnapshot) -> Result<SckCaptureTarget>,
) -> Result<SckObservationProbeReport> {
    if !error.is_null() {
        return Err(fail_closed(&format!(
            "discovery failed: {}",
            super::shareable_request::retained_error_message(error)
        )));
    }
    let content = unsafe { Retained::retain(content) }
        .ok_or_else(|| fail_closed("discovery returned no content"))?;
    observe_from_content(content, request, selector)
}

pub(super) fn observe_from_content(
    content: Retained<SCShareableContent>,
    request: SckObservationProbeRequest,
    selector: &impl Fn(&SckShareableContentSnapshot) -> Result<SckCaptureTarget>,
) -> Result<SckObservationProbeReport> {
    let snapshot = snapshot_shareable_content(&content)?;
    let target = selector(&snapshot)?;
    let plan = build_stream_capture_plan(&content, &target)?;
    ensure_screen_capture_access()?;
    let stream = SckFrameMetadataStreamRegistration::from_plan(&plan)?;
    let frames = stream.observe_for(request.capture_duration, request.completion_timeout)?;
    let vision = stream.vision_observations()?;
    let live_mask_evidence = stream.live_mask_evidence()?;
    let accessibility = super::ax_probe::observe_accessibility(&target, &frames)?;
    Ok(SckObservationProbeReport {
        target,
        frame_size: plan.frame_size,
        frames,
        accessibility,
        vision,
        live_mask_evidence,
    })
}

pub(super) fn fail_closed(reason: &str) -> AppError {
    AppError::InvalidConfig(format!(
        "Secure Share ScreenCaptureKit observation probe {reason}"
    ))
}

pub(super) fn ensure_screen_capture_access() -> Result<()> {
    if super::screen_permission::screen_capture_access_granted() {
        Ok(())
    } else {
        Err(fail_closed("Screen Recording permission is missing"))
    }
}
