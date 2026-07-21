use dropsquash_core::{MaskPlan, MaskPlanDraft, MaskPolicy, VerificationExpectations};
use dropsquash_platform::SckObservationProbeReport;

pub(super) fn from_report(window_id: u32, report: &SckObservationProbeReport) -> MaskPlan {
    MaskPlanDraft {
        capture_id: format!("build-week-alpha-window-{window_id}"),
        frame_size: report.frame_size,
        frames: report.frames.clone(),
        accessibility: report.accessibility.clone(),
        vision: report.vision.clone(),
        temporal: Vec::new(),
        policy: MaskPolicy::StrictReveal,
        verification_expectations: VerificationExpectations {
            no_audio: true,
            strip_metadata: true,
            verification_policy_version: "build-week-alpha-preview-v1".to_string(),
        },
    }
    .into_mask_plan()
}
