use dropsquash_core::FrameSize;
use dropsquash_platform::{
    SckCaptureTarget, SckCaptureTargetKind, SckLiveMaskEvidence, SckRecordingProbeReport,
};

use super::super::super::paths::RecordingPaths;
use super::super::super::selection::NativeWindowSelection;

pub(super) fn build(
    selection: NativeWindowSelection,
    width: u32,
    height: u32,
    frames: Vec<dropsquash_core::CaptureFrameMetadata>,
    accessibility: Vec<dropsquash_core::AxObservation>,
    vision: Vec<dropsquash_core::VisionObservation>,
    temporal: Vec<dropsquash_core::TemporalObservation>,
    paths: &RecordingPaths,
) -> SckRecordingProbeReport {
    SckRecordingProbeReport {
        target: SckCaptureTarget {
            kind: SckCaptureTargetKind::Window,
            id: selection.window_id,
            frame: selection.frame,
            owner_pid: Some(selection.owner_pid),
        },
        frame_size: FrameSize { width, height },
        frames,
        accessibility,
        vision,
        temporal,
        live_mask_evidence: SckLiveMaskEvidence::default(),
        temporary_output: paths.partial.clone(),
    }
}
