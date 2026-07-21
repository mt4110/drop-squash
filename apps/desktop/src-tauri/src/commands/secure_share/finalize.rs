use dropsquash_core::{
    default_evidence_signing_key_path, required_capture_continuity_watches,
    strict_shield_exposure_coverage, MaskMode, MaskPlanDraft, MaskPolicy, SecureShareOptions,
    VerificationExpectations,
};
use dropsquash_encoder::verify_mask_plan_output;
use dropsquash_fileguard::verify_secure_share_evidence;
use dropsquash_platform::SckRecordingProbeReport;
use serde::Serialize;

use super::paths::RecordingPaths;
use super::publication::PublicationPermit;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecureShareRecordingDto {
    pub output_path: String,
    pub mask_plan_path: String,
    pub frame_count: usize,
    pub accessibility_observation_count: usize,
    pub focused_text_observation_count: usize,
    pub vision_observation_count: usize,
    pub temporal_observation_count: usize,
    pub masked_region_count: usize,
    pub fully_masked_frame_count: usize,
    pub continuity_attested: bool,
}

pub(super) fn recording(
    report: dropsquash_core::Result<SckRecordingProbeReport>,
    paths: RecordingPaths,
    permit: PublicationPermit,
) -> Result<SecureShareRecordingDto, String> {
    let result = report
        .map_err(|error| error.user_message())
        .and_then(|report| verified(report, &paths, &permit));
    if result.is_err() {
        permit.discard(&paths);
    }
    result
}

fn verified(
    report: SckRecordingProbeReport,
    paths: &RecordingPaths,
    permit: &PublicationPermit,
) -> Result<SecureShareRecordingDto, String> {
    let mut plan = MaskPlanDraft {
        capture_id: "secure-share-recording-local".to_string(),
        frame_size: report.frame_size,
        frames: report.frames.clone(),
        accessibility: report.accessibility,
        vision: report.vision.clone(),
        temporal: report.temporal,
        policy: MaskPolicy::StrictReveal,
        verification_expectations: VerificationExpectations {
            no_audio: true,
            strip_metadata: true,
            verification_policy_version: "phase5-native-bridge-v1".to_string(),
        },
    }
    .into_mask_plan();
    plan.schema_version = 2;
    plan.audit.capture_continuity_attested = Some(true);
    plan.audit.capture_backend = Some("apple_native_capture_v1".to_string());
    plan.audit.record_native_destruction(
        report.live_mask_evidence.masked_frame_count,
        report.live_mask_evidence.masked_rect_count,
        report.live_mask_evidence.verified_pixel_count,
    );
    plan.audit.capture_continuity_watches = required_capture_continuity_watches()
        .iter()
        .map(|watch| (*watch).to_string())
        .collect();
    plan.audit.exposure_coverage = strict_shield_exposure_coverage();
    let regions = plan.frames.iter().map(|frame| frame.regions.len()).sum();
    let fully_masked_frames = plan
        .frames
        .iter()
        .filter(|frame| {
            frame.regions.iter().any(|region| {
                region.rect.x == 0
                    && region.rect.y == 0
                    && region.rect.width == plan.frame_size.width
                    && region.rect.height == plan.frame_size.height
            })
        })
        .count();
    verify_mask_plan_output(
        &paths.partial,
        &SecureShareOptions {
            mask_mode: MaskMode::SolidBlack,
            mask_rects: Vec::new(),
            mask_plan: Some(plan.clone()),
        },
    )
    .map_err(|error| error.user_message())?;
    let temporary = crate::commands::qa::secure_share::alpha::evidence::prepare(
        &paths.partial,
        &paths.final_path,
        &plan,
        &default_evidence_signing_key_path(),
    )
    .map_err(|error| error.user_message())?;
    let mask_plan_path = permit.publish(&temporary, paths)?;
    verify_secure_share_evidence(&paths.final_path, &mask_plan_path)
        .map_err(|error| error.user_message())?;
    Ok(SecureShareRecordingDto {
        output_path: paths.final_path.display().to_string(),
        mask_plan_path: mask_plan_path.display().to_string(),
        frame_count: report.frames.len(),
        accessibility_observation_count: plan.audit.accessibility_observation_count,
        focused_text_observation_count: plan.audit.focused_text_observation_count,
        vision_observation_count: report.vision.len(),
        temporal_observation_count: plan.audit.temporal_observation_count,
        masked_region_count: regions,
        fully_masked_frame_count: fully_masked_frames,
        continuity_attested: plan.audit.capture_continuity_attested == Some(true),
    })
}
