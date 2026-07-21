use std::path::PathBuf;
use std::sync::mpsc::sync_channel;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use dropsquash_core::MaskPolicy;
use dropsquash_platform::{record_window_once_callback_with_policy, SckObservationProbeRequest};
use serde::Serialize;

mod experimental;
mod finalize;
mod residual;
mod truth;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecureShareRecordingDto {
    pub output_path: String,
    pub mask_plan_path: String,
    pub frame_count: usize,
    pub accessibility_observation_count: usize,
    pub vision_observation_count: usize,
    pub cross_source_overlap_count: usize,
    pub accessibility_only_count: usize,
    pub vision_only_count: usize,
    pub masked_region_count: usize,
    pub fully_masked_frame_count: usize,
    pub max_masked_area_ppm: u32,
    pub mean_masked_area_ppm: u32,
    pub min_truth_covered_ppm: Option<u32>,
    pub max_mask_outside_truth_ppm: Option<u32>,
    pub decoded_text_observation_count: u64,
    pub decoded_unmasked_text_count: u64,
    pub continuity_attested: bool,
}

#[tauri::command(rename_all = "camelCase")]
pub async fn secure_share_alpha_record(
    app: tauri::AppHandle,
    window_id: u32,
    output_dir: String,
) -> Result<SecureShareRecordingDto, String> {
    let paths = recording_paths(PathBuf::from(output_dir))?;
    let completion_paths = paths.clone();
    let timeout_cleanup_paths = paths.clone();
    let policy = qa_policy();
    let (sender, receiver) = sync_channel(1);
    app.run_on_main_thread(move || {
        record_window_once_callback_with_policy(
            window_id,
            paths.partial.clone(),
            request(),
            policy,
            move |result| {
                let paths = completion_paths.clone();
                if sender.send((result, paths.clone())).is_err() {
                    let _ = std::fs::remove_file(&paths.partial);
                }
            },
        );
    })
    .map_err(|error| format!("Secure Share recording start failed: {error}"))?;
    let recorded = tauri::async_runtime::spawn_blocking(move || {
        receiver
            .recv_timeout(Duration::from_secs(15))
            .map_err(|_| "Secure Share recording timed out".to_string())
    })
    .await
    .map_err(|error| format!("Secure Share recording task failed: {error}"))?;
    recorded
        .and_then(|record| finalize::recording(record, policy))
        .inspect_err(|_| {
            let _ = std::fs::remove_file(&timeout_cleanup_paths.partial);
        })
}

fn request() -> SckObservationProbeRequest {
    SckObservationProbeRequest {
        discovery_timeout: Duration::from_secs(10),
        completion_timeout: Duration::from_secs(10),
        capture_duration: qa_capture_duration(),
    }
}

fn qa_capture_duration() -> Duration {
    let milliseconds = std::env::var("DROP_SQUASH_QA_SCK_CAPTURE_DURATION_MS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(3_000);
    Duration::from_millis(milliseconds)
}

fn qa_policy() -> MaskPolicy {
    (std::env::var("DROP_SQUASH_QA_SCK_RECORD_POLICY")
        .ok()
        .as_deref()
        == Some("strict_reveal"))
    .then_some(MaskPolicy::StrictReveal)
    .unwrap_or(MaskPolicy::SmartMask)
}

#[derive(Clone)]
pub(super) struct RecordingPaths {
    pub(super) partial: PathBuf,
    pub(super) final_path: PathBuf,
}

fn recording_paths(output_dir: PathBuf) -> Result<RecordingPaths, String> {
    std::fs::create_dir_all(&output_dir).map_err(|error| error.to_string())?;
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?
        .as_millis();
    let stem = format!("secure-share-alpha-{stamp}-{}", std::process::id());
    Ok(RecordingPaths {
        partial: output_dir.join(format!(".{stem}.partial.mp4")),
        final_path: output_dir.join(format!("{stem}.mp4")),
    })
}
