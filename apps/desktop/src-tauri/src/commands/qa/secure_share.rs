use std::time::Duration;

use dropsquash_platform::{
    observe_window_once, observe_window_once_callback, SckObservationProbeReport,
    SckObservationProbeRequest,
};
use serde::Serialize;

pub mod alpha;
mod mask_plan_preview;
mod startup;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecureShareObservationDto {
    pub window_id: u32,
    pub frame_width: u32,
    pub frame_height: u32,
    pub frame_count: usize,
    pub ax_observation_count: usize,
    pub vision_observation_count: usize,
    pub first_frame_time_ns: Option<u64>,
    pub last_frame_time_ns: Option<u64>,
    pub mask_plan_preview: dropsquash_core::MaskPlan,
    pub black_fill_proof: Option<dropsquash_encoder::MaskPlanBlackFillProof>,
    pub black_fill_proof_error: Option<String>,
    pub live_masked_frame_count: usize,
    pub live_masked_rect_count: usize,
    pub live_verified_pixel_count: usize,
}

#[tauri::command(rename_all = "camelCase")]
pub async fn manual_qa_secure_share_observe_window(
    window_id: u32,
    capture_ms: u64,
    timeout_ms: u64,
) -> Result<SecureShareObservationDto, String> {
    if std::env::var("DROP_SQUASH_QA_SCK_OBSERVE").ok().as_deref() != Some("1") {
        return Err("set DROP_SQUASH_QA_SCK_OBSERVE=1 to run Secure Share QA observation".into());
    }
    tauri::async_runtime::spawn_blocking(move || observe_window(window_id, capture_ms, timeout_ms))
        .await
        .map_err(|error| format!("Secure Share QA observation task failed: {error}"))?
}

fn observe_window(
    window_id: u32,
    capture_ms: u64,
    timeout_ms: u64,
) -> Result<SecureShareObservationDto, String> {
    let request = SckObservationProbeRequest {
        discovery_timeout: Duration::from_millis(timeout_ms),
        completion_timeout: Duration::from_millis(timeout_ms),
        capture_duration: Duration::from_millis(capture_ms),
    };
    let report = observe_window_once(window_id, request).map_err(|error| error.user_message())?;
    Ok(dto(window_id, report))
}

pub(super) fn observe_window_callback(
    window_id: u32,
    capture_ms: u64,
    timeout_ms: u64,
    callback: impl Fn(Result<SecureShareObservationDto, String>) + Send + Sync + 'static,
) {
    let request = SckObservationProbeRequest {
        discovery_timeout: Duration::from_millis(timeout_ms),
        completion_timeout: Duration::from_millis(timeout_ms),
        capture_duration: Duration::from_millis(capture_ms),
    };
    observe_window_once_callback(window_id, request, move |result| {
        callback(
            result
                .map(|report| dto(window_id, report))
                .map_err(|error| error.user_message()),
        );
    });
}

fn dto(window_id: u32, report: SckObservationProbeReport) -> SecureShareObservationDto {
    let mask_plan_preview = mask_plan_preview::from_report(window_id, &report);
    let black_fill = dropsquash_encoder::prove_mask_plan_solid_black_fill(&mask_plan_preview);
    let (black_fill_proof, black_fill_proof_error) = match black_fill {
        Ok(proof) => (Some(proof), None),
        Err(error) => (None, Some(error.user_message())),
    };
    SecureShareObservationDto {
        window_id,
        frame_width: report.frame_size.width,
        frame_height: report.frame_size.height,
        frame_count: report.frames.len(),
        ax_observation_count: report.accessibility.len(),
        vision_observation_count: report.vision.len(),
        first_frame_time_ns: report
            .frames
            .first()
            .map(|frame| frame.presentation_time_ns),
        last_frame_time_ns: report.frames.last().map(|frame| frame.presentation_time_ns),
        mask_plan_preview,
        black_fill_proof,
        black_fill_proof_error,
        live_masked_frame_count: report.live_mask_evidence.masked_frame_count,
        live_masked_rect_count: report.live_mask_evidence.masked_rect_count,
        live_verified_pixel_count: report.live_mask_evidence.verified_pixel_count,
    }
}

pub fn run_startup_observation(app: tauri::AppHandle) {
    startup::run(app);
}
