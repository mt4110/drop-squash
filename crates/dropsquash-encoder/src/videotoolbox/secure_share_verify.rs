use std::path::Path;

use dropsquash_core::{AppError, Result, SecureShareOptions};
use objc2_av_foundation::{
    AVAssetReader, AVAssetReaderStatus, AVAssetReaderTrackOutput, AVURLAsset,
};

use super::pixel_buffer::sample_frame_size;
use super::pixel_buffer::verify::verify_black_regions;
use super::secure_share::video_track;
use super::secure_share_settings::reader_output_settings;
use super::session::file_url;

mod asset;
mod experimental;
mod frame_count;
mod timeline;

pub(crate) use experimental::verify_experimental_masked_output;

pub(crate) fn verify_masked_output(path: &Path, options: &SecureShareOptions) -> Result<()> {
    let url = file_url(path)?;
    let asset = unsafe { AVURLAsset::URLAssetWithURL_options(&url, None) };
    asset::verify_asset_constraints(path, &asset, options)?;
    let track = video_track(&asset)?;
    let reader = unsafe { AVAssetReader::assetReaderWithAsset_error(&asset) }
        .map_err(|error| reader_error("Secure Share verification could not open output", &error))?;
    let output = unsafe {
        AVAssetReaderTrackOutput::assetReaderTrackOutputWithTrack_outputSettings(
            &track,
            Some(&reader_output_settings()),
        )
    };
    unsafe { reader.addOutput(&output) };
    if !unsafe { reader.startReading() } {
        return Err(status_error(
            "Secure Share verification could not start",
            &reader,
        ));
    }
    let mut frame = 0u64;
    let mut timeline = timeline::DecodedTimeline::default();
    while let Some(sample) = unsafe { output.copyNextSampleBuffer() } {
        if unsafe { sample.num_samples() } == 0 {
            continue;
        }
        timeline.accept(unsafe { sample.presentation_time_stamp() })?;
        let options =
            crate::secure_share::options_for_frame(options, frame, sample_frame_size(&sample)?)?;
        verify_black_regions(&sample, &options)?;
        frame += 1;
    }
    if unsafe { reader.status() } != AVAssetReaderStatus::Completed {
        return Err(status_error(
            "Secure Share verification did not complete",
            &reader,
        ));
    }
    frame_count::verify(
        frame,
        options.mask_plan.as_ref().map(|plan| plan.frames.len()),
    )?;
    Ok(())
}

fn reader_error(prefix: &str, error: &objc2_foundation::NSError) -> AppError {
    AppError::Encoder(format!("{prefix}: {}", error.localizedDescription()))
}

fn status_error(prefix: &str, reader: &AVAssetReader) -> AppError {
    unsafe { reader.error() }.map_or_else(
        || AppError::Encoder(prefix.to_string()),
        |error| reader_error(prefix, &error),
    )
}
