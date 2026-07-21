use std::path::Path;

use dropsquash_core::{
    selective_mask_precision, selective_mask_precision_for_frames, AppError, MaskPlan, PixelRect,
    Result, SelectiveMaskPrecision,
};
use serde::Deserialize;

const FIXTURE_TRUTH_PATH: &str = "DROP_SQUASH_QA_SENSITIVE_TRUTH_PATH";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FixtureTruth {
    frame_width: u32,
    frame_height: u32,
    sensitive: Vec<PixelRect>,
    #[serde(default)]
    timed_sensitive: Vec<TimedSensitive>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TimedSensitive {
    start_ns: u64,
    end_ns: u64,
    rect: PixelRect,
}

pub(super) fn precision(plan: &MaskPlan) -> Result<Option<SelectiveMaskPrecision>> {
    let Some(path) = std::env::var_os(FIXTURE_TRUTH_PATH) else {
        return Ok(None);
    };
    let truth: FixtureTruth = serde_json::from_slice(&std::fs::read(Path::new(&path))?)?;
    if truth.frame_width != plan.frame_size.width || truth.frame_height != plan.frame_size.height {
        return Err(invalid("frame size differs from the fixture truth"));
    }
    if truth.sensitive.is_empty() {
        return Err(invalid("fixture truth has no sensitive rectangles"));
    }
    if truth.timed_sensitive.is_empty() {
        return Ok(Some(selective_mask_precision(plan, &truth.sensitive)));
    }
    let per_frame = plan
        .frames
        .iter()
        .map(|frame| {
            let mut sensitive = truth.sensitive.clone();
            sensitive.extend(
                truth
                    .timed_sensitive
                    .iter()
                    .filter(|item| {
                        item.start_ns <= frame.presentation_time_ns
                            && frame.presentation_time_ns < item.end_ns
                    })
                    .map(|item| item.rect),
            );
            sensitive
        })
        .collect::<Vec<_>>();
    selective_mask_precision_for_frames(plan, &per_frame)
        .map(Some)
        .ok_or_else(|| invalid("has no frame-specific sensitive rectangles"))
}

fn invalid(reason: &str) -> AppError {
    AppError::InvalidConfig(format!("Secure Share experimental fixture truth {reason}"))
}

#[cfg(test)]
mod tests;
