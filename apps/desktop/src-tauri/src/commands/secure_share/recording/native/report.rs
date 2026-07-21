use dropsquash_core::FrameSize;
use dropsquash_platform::{
    SckCaptureTarget, SckCaptureTargetKind, SckLiveMaskEvidence, SckRecordingProbeReport,
};

use super::super::super::paths::RecordingPaths;
use super::super::super::selection::NativeWindowSelection;

pub(super) struct ReportInput<'a> {
    pub(super) selection: NativeWindowSelection,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) frames: Vec<dropsquash_core::CaptureFrameMetadata>,
    pub(super) accessibility: Vec<dropsquash_core::AxObservation>,
    pub(super) vision: Vec<dropsquash_core::VisionObservation>,
    pub(super) temporal: Vec<dropsquash_core::TemporalObservation>,
    pub(super) paths: &'a RecordingPaths,
}

pub(super) fn build(input: ReportInput<'_>) -> SckRecordingProbeReport {
    SckRecordingProbeReport {
        target: SckCaptureTarget {
            kind: SckCaptureTargetKind::Window,
            id: input.selection.window_id,
            frame: input.selection.frame,
            owner_pid: Some(input.selection.owner_pid),
        },
        frame_size: FrameSize {
            width: input.width,
            height: input.height,
        },
        frames: input.frames,
        accessibility: input.accessibility,
        vision: input.vision,
        temporal: input.temporal,
        live_mask_evidence: SckLiveMaskEvidence::default(),
        temporary_output: input.paths.partial.clone(),
    }
}
