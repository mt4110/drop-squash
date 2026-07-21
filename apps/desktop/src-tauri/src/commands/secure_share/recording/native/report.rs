use dropsquash_core::FrameSize;
use dropsquash_platform::{
    NativeDestructionEvidence, SckCaptureTarget, SckCaptureTargetKind, SckLiveMaskEvidence,
    SckRecordingProbeReport,
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
    pub(super) destruction: Vec<NativeDestructionEvidence>,
    pub(super) paths: &'a RecordingPaths,
}

pub(super) fn build(input: ReportInput<'_>) -> Result<SckRecordingProbeReport, String> {
    let evidence = strict_shield_evidence(&input)?;
    Ok(SckRecordingProbeReport {
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
        live_mask_evidence: evidence,
        temporary_output: input.paths.partial.clone(),
    })
}

fn strict_shield_evidence(input: &ReportInput<'_>) -> Result<SckLiveMaskEvidence, String> {
    let matches = input.destruction.len() == input.frames.len()
        && input.destruction.iter().enumerate().all(|(index, item)| {
            item.frame_index == index as u64
                && item.policy == 1
                && item.region_count == 1
                && item.output_width == input.width
                && item.output_height == input.height
        });
    if !matches {
        return Err("Secure Share native destruction evidence is incomplete".to_string());
    }
    Ok(SckLiveMaskEvidence {
        masked_frame_count: input.destruction.len(),
        masked_rect_count: input.destruction.len(),
        verified_pixel_count: input.destruction.len(),
        ..SckLiveMaskEvidence::default()
    })
}

#[cfg(test)]
mod tests;
