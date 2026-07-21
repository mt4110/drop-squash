use std::path::Path;

use dropsquash_core::{mask_options_for_frame, AppError, MaskPolicy, Result, SecureShareOptions};
use objc2_av_foundation::{
    AVAssetReader, AVAssetReaderStatus, AVAssetReaderTrackOutput, AVURLAsset,
};

use super::super::pixel_buffer::sample_frame_size;
use super::super::secure_share::video_track;
use super::super::secure_share_settings::reader_output_settings;
use super::super::session::file_url;
use super::{asset, frame_count};

pub(crate) fn verify_experimental_masked_output(
    path: &Path,
    options: &SecureShareOptions,
) -> Result<()> {
    let plan = options
        .mask_plan
        .as_ref()
        .filter(|plan| plan.policy == MaskPolicy::SmartMask)
        .ok_or_else(|| invalid("requires a SmartMask plan"))?;
    let url = file_url(path)?;
    let asset = unsafe { AVURLAsset::URLAssetWithURL_options(&url, None) };
    asset::verify_asset_constraints(path, &asset, options)?;
    let track = video_track(&asset)?;
    let reader = unsafe { AVAssetReader::assetReaderWithAsset_error(&asset) }
        .map_err(|error| reader_error("could not open output", &error))?;
    let output = unsafe {
        AVAssetReaderTrackOutput::assetReaderTrackOutputWithTrack_outputSettings(
            &track,
            Some(&reader_output_settings()),
        )
    };
    unsafe { reader.addOutput(&output) };
    if !unsafe { reader.startReading() } {
        return Err(status_error("could not start", &reader));
    }
    let mut frame = 0u64;
    let mut timeline = super::timeline::DecodedTimeline::default();
    while let Some(sample) = unsafe { output.copyNextSampleBuffer() } {
        timeline.accept(unsafe { sample.presentation_time_stamp() })?;
        let size = sample_frame_size(&sample)?;
        if size != plan.frame_size {
            return Err(invalid("frame size differs from the SmartMask plan"));
        }
        let resolved = mask_options_for_frame(plan, frame, options.mask_mode)
            .ok_or_else(|| invalid("plan is missing a decoded frame"))?;
        if resolved.mask_rects.is_empty() {
            return Err(invalid("plan frame has no destructive regions"));
        }
        super::super::pixel_buffer::verify::verify_black_regions(&sample, &resolved)?;
        frame += 1;
    }
    if unsafe { reader.status() } != AVAssetReaderStatus::Completed {
        return Err(status_error("did not complete", &reader));
    }
    frame_count::verify(frame, Some(plan.frames.len()))
}

fn reader_error(prefix: &str, error: &objc2_foundation::NSError) -> AppError {
    AppError::Encoder(format!(
        "experimental MaskPlan verification {prefix}: {}",
        error.localizedDescription()
    ))
}

fn status_error(prefix: &str, reader: &AVAssetReader) -> AppError {
    unsafe { reader.error() }.map_or_else(
        || AppError::Encoder(format!("experimental MaskPlan verification {prefix}")),
        |error| reader_error(prefix, &error),
    )
}

fn invalid(reason: &str) -> AppError {
    AppError::InvalidConfig(format!("experimental MaskPlan verification {reason}"))
}
