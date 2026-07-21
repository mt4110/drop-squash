use std::path::PathBuf;

use dropsquash_core::{AxObservation, FrameSize, Result, TemporalObservation, VisionObservation};

use super::super::{
    FrameMetadataProvider, SckCaptureTarget, SckFrameMetadataStreamRegistration,
    SckObservationProbeRequest, SckRecordingProbeReport, SckWindowSelection,
};

pub(super) fn report(
    stream: &SckFrameMetadataStreamRegistration,
    request: SckObservationProbeRequest,
    selection: &SckWindowSelection,
    target: SckCaptureTarget,
    frame_size: FrameSize,
    output_path: PathBuf,
    accessibility: Vec<AxObservation>,
) -> Result<SckRecordingProbeReport> {
    super::revalidation::after_stop(selection, request.discovery_timeout)?;
    let frames = stream.frames()?;
    let vision: Vec<VisionObservation> = stream.vision_observations()?;
    let live_mask_evidence = stream.live_mask_evidence()?;
    let temporary_output = stream
        .finish_recording()?
        .ok_or_else(|| super::super::recording_probe::fail_closed("no output"))?;
    if temporary_output != output_path {
        return Err(super::super::recording_probe::fail_closed(
            "unexpected output path",
        ));
    }
    Ok(SckRecordingProbeReport {
        target,
        frame_size,
        frames,
        accessibility,
        vision,
        temporal: Vec::<TemporalObservation>::new(),
        live_mask_evidence,
        temporary_output,
    })
}
