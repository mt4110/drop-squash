use dropsquash_core::{AxObservation, FrameSize, MaskPolicy, OutputSize, Result};
use objc2::rc::Retained;
use objc2_screen_capture_kit::SCShareableContent;
use std::path::PathBuf;

use crate::secure_share::{
    select_attested_window_target, snapshot_shareable_content, SckCaptureTarget,
    SckFrameMetadataStreamRegistration, SckWindowSelection,
};

type Prepared = (
    SckCaptureTarget,
    FrameSize,
    Vec<AxObservation>,
    SckFrameMetadataStreamRegistration,
);

pub(super) fn prepare(
    selection: &SckWindowSelection,
    output_path: PathBuf,
    output_size: OutputSize,
    policy: MaskPolicy,
    content: Retained<SCShareableContent>,
) -> Result<Prepared> {
    let snapshot = snapshot_shareable_content(&content)?;
    let target = select_attested_window_target(&snapshot, selection)?;
    super::super::observation_probe::ensure_screen_capture_access()?;
    let plan = super::super::build_stream_capture_plan_for_output(&content, &target, output_size)?;
    let accessibility =
        super::super::ax_probe::observe_for_recording(&target, plan.frame_size, policy)?;
    let stream = SckFrameMetadataStreamRegistration::from_plan_recording(
        &plan,
        output_path,
        accessibility.clone(),
        policy,
    )?;
    Ok((target, plan.frame_size, accessibility, stream))
}
