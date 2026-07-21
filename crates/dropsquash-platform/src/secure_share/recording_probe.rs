use std::path::PathBuf;
use std::sync::mpsc::sync_channel;

use super::{
    select_explicit_window_target, snapshot_shareable_content, SckCaptureTarget,
    SckLiveMaskEvidence, SckObservationProbeRequest, SckWindowSelection,
};
use dropsquash_core::{
    AppError, AxObservation, CaptureFrameMetadata, FrameSize, MaskPolicy, OutputSize, Result,
    TemporalObservation, VisionObservation,
};
use objc2::rc::Retained;
use objc2_screen_capture_kit::SCShareableContent;

mod discovery;
pub use discovery::record_window_once;

#[derive(Debug, Clone, PartialEq)]
pub struct SckRecordingProbeReport {
    pub target: SckCaptureTarget,
    pub frame_size: FrameSize,
    pub frames: Vec<CaptureFrameMetadata>,
    pub accessibility: Vec<AxObservation>,
    pub vision: Vec<VisionObservation>,
    pub temporal: Vec<TemporalObservation>,
    pub live_mask_evidence: SckLiveMaskEvidence,
    pub temporary_output: PathBuf,
}

pub(super) fn record_from_content(
    window_id: u32,
    output_path: PathBuf,
    request: SckObservationProbeRequest,
    content: Retained<SCShareableContent>,
) -> Result<SckRecordingProbeReport> {
    record_from_content_with_policy(
        window_id,
        output_path,
        request,
        MaskPolicy::StrictReveal,
        content,
    )
}

pub(super) fn record_from_content_with_policy(
    window_id: u32,
    output_path: PathBuf,
    request: SckObservationProbeRequest,
    policy: MaskPolicy,
    content: Retained<SCShareableContent>,
) -> Result<SckRecordingProbeReport> {
    let snapshot = snapshot_shareable_content(&content)?;
    let target = select_explicit_window_target(&snapshot, window_id)?;
    let selection = attested_selection(&snapshot, window_id, target)?;
    let (stop, receiver) = sync_channel(1);
    let duration = request.capture_duration;
    std::thread::spawn(move || {
        std::thread::sleep(duration);
        let _ = stop.send(());
    });
    super::recording_session::record_until_stopped_from_content(
        selection,
        output_path,
        OutputSize::Auto,
        policy,
        request,
        content,
        receiver,
        |_| {},
    )
}

fn attested_selection(
    snapshot: &super::SckShareableContentSnapshot,
    window_id: u32,
    target: SckCaptureTarget,
) -> Result<SckWindowSelection> {
    let window = snapshot
        .windows
        .iter()
        .find(|window| window.window_id == window_id)
        .ok_or_else(|| fail_closed("selected window disappeared"))?;
    Ok(SckWindowSelection {
        window_id,
        owner_pid: target
            .owner_pid
            .ok_or_else(|| fail_closed("selected window has no owner"))?,
        title: window.title.clone(),
        frame: target.frame,
    })
}

pub(super) fn fail_closed(reason: &str) -> AppError {
    AppError::InvalidConfig(format!("Secure Share recording {reason}"))
}

#[cfg(test)]
#[path = "recording_probe/tests.rs"]
mod tests;
