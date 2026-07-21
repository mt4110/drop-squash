use dropsquash_core::{
    destructive_coverage, MaskMode, MaskPlanDraft, MaskPolicy, SecureShareOptions,
    VerificationExpectations,
};
use dropsquash_encoder::{verify_experimental_mask_plan_output, verify_mask_plan_output};
use dropsquash_platform::{decoded_text_residual_report, SckRecordingProbeReport};

use super::{experimental, residual, truth, RecordingPaths, SecureShareRecordingDto};

pub(super) fn recording(
    (report, paths): (
        dropsquash_core::Result<SckRecordingProbeReport>,
        RecordingPaths,
    ),
    policy: MaskPolicy,
) -> Result<SecureShareRecordingDto, String> {
    let result = report
        .map_err(|error| error.user_message())
        .and_then(|report| verified(report, &paths, policy));
    if result.is_err() {
        let _ = std::fs::remove_file(&paths.partial);
    }
    result
}

fn verified(
    report: SckRecordingProbeReport,
    paths: &RecordingPaths,
    policy: MaskPolicy,
) -> Result<SecureShareRecordingDto, String> {
    let plan = MaskPlanDraft {
        capture_id: "secure-share-recording-local".to_string(),
        frame_size: report.frame_size,
        frames: report.frames.clone(),
        accessibility: report.accessibility,
        vision: report.vision.clone(),
        temporal: Vec::new(),
        policy,
        verification_expectations: VerificationExpectations {
            no_audio: true,
            strip_metadata: true,
            verification_policy_version: "phase5-alpha-v1".to_string(),
        },
    }
    .into_mask_plan();
    let regions = plan.frames.iter().map(|frame| frame.regions.len()).sum();
    let coverage = destructive_coverage(&plan);
    let precision = truth::precision(&plan).map_err(|error| error.user_message())?;
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
    let options = SecureShareOptions {
        mask_mode: MaskMode::SolidBlack,
        mask_rects: Vec::new(),
        mask_plan: Some(plan.clone()),
    };
    match policy {
        MaskPolicy::SmartMask => verify_experimental_mask_plan_output(&paths.partial, &options),
        MaskPolicy::StrictReveal => verify_mask_plan_output(&paths.partial, &options),
    }
    .map_err(|error| error.user_message())?;
    let residual = decoded_text_residual_report(&paths.partial, &plan)
        .map_err(|error| error.user_message())?;
    residual::reject_unmasked_text(residual.unmasked_observation_count)?;
    let mask_plan_path = experimental::save(
        &paths.final_path,
        &plan,
        residual.text_observation_count,
        residual.unmasked_observation_count,
    )
    .map_err(|error| error.user_message())?;
    if let Err(error) = std::fs::rename(&paths.partial, &paths.final_path) {
        let _ = std::fs::remove_file(&mask_plan_path);
        return Err(error.to_string());
    }
    Ok(SecureShareRecordingDto {
        output_path: paths.final_path.display().to_string(),
        mask_plan_path: mask_plan_path.display().to_string(),
        frame_count: report.frames.len(),
        accessibility_observation_count: plan.audit.accessibility_observation_count,
        vision_observation_count: report.vision.len(),
        cross_source_overlap_count: plan.audit.cross_source_overlap_count,
        accessibility_only_count: plan.audit.accessibility_only_count,
        vision_only_count: plan.audit.vision_only_count,
        masked_region_count: regions,
        fully_masked_frame_count: fully_masked_frames,
        max_masked_area_ppm: coverage.max_frame_ppm,
        mean_masked_area_ppm: coverage.mean_frame_ppm,
        min_truth_covered_ppm: precision.map(|value| value.min_truth_covered_ppm),
        max_mask_outside_truth_ppm: precision.map(|value| value.max_mask_outside_truth_ppm),
        decoded_text_observation_count: residual.text_observation_count,
        decoded_unmasked_text_count: residual.unmasked_observation_count,
        continuity_attested: plan.audit.capture_continuity_attested == Some(true),
    })
}
