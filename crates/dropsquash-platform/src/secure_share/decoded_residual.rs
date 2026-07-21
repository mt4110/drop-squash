use std::path::Path;

use dropsquash_core::{mask_options_for_frame, AppError, FrameSize, MaskMode, MaskPlan, Result};
use objc2::runtime::AnyObject;
use objc2_av_foundation::{
    AVAssetReader, AVAssetReaderStatus, AVAssetReaderTrackOutput, AVMediaCharacteristicVisual,
    AVURLAsset,
};
use objc2_core_video::kCVPixelFormatType_32BGRA;
use objc2_foundation::{ns_string, NSDictionary, NSNumber, NSURL};

use super::attachments::NativeSampleBuffer;
use super::native_vision_observe::observe_decoded_sample_buffer;

mod geometry;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodedTextResidualReport {
    pub decoded_frame_count: u64,
    pub text_observation_count: u64,
    pub unmasked_observation_count: u64,
}

pub fn decoded_text_residual_report(
    path: &Path,
    plan: &MaskPlan,
) -> Result<DecodedTextResidualReport> {
    let url = NSURL::from_file_path(path).ok_or_else(|| invalid("output path is invalid"))?;
    let asset = unsafe { AVURLAsset::URLAssetWithURL_options(&url, None) };
    let track = video_track(&asset)?;
    let reader = unsafe { AVAssetReader::assetReaderWithAsset_error(&asset) }
        .map_err(|error| AppError::Encoder(error.localizedDescription().to_string()))?;
    let output = unsafe {
        AVAssetReaderTrackOutput::assetReaderTrackOutputWithTrack_outputSettings(
            &track,
            Some(&reader_settings()),
        )
    };
    unsafe { reader.addOutput(&output) };
    if !unsafe { reader.startReading() } {
        return Err(reader_error(&reader));
    }
    let mut report = DecodedTextResidualReport {
        decoded_frame_count: 0,
        text_observation_count: 0,
        unmasked_observation_count: 0,
    };
    while let Some(sample) = unsafe { output.copyNextSampleBuffer() } {
        let size = frame_size(&sample)?;
        if size != plan.frame_size {
            return Err(invalid("decoded frame size differs from plan"));
        }
        let options =
            mask_options_for_frame(plan, report.decoded_frame_count, MaskMode::SolidBlack)
                .ok_or_else(|| invalid("decoded frame exceeds plan"))?;
        let observations = observe_decoded_sample_buffer(&sample, size)?;
        report.text_observation_count += observations.len() as u64;
        report.unmasked_observation_count += observations
            .iter()
            .filter(|observation| {
                !options
                    .mask_rects
                    .iter()
                    .any(|mask| geometry::contains(*mask, observation.rect))
            })
            .count() as u64;
        report.decoded_frame_count += 1;
    }
    if unsafe { reader.status() } != AVAssetReaderStatus::Completed {
        return Err(reader_error(&reader));
    }
    if report.decoded_frame_count != plan.frames.len() as u64 {
        return Err(invalid("decoded frame count differs from plan"));
    }
    Ok(report)
}

fn reader_settings() -> objc2::rc::Retained<NSDictionary<objc2_foundation::NSString, AnyObject>> {
    NSDictionary::from_retained_objects(
        &[ns_string!("PixelFormatType")],
        &[NSNumber::new_u32(kCVPixelFormatType_32BGRA).into()],
    )
}

fn video_track(
    asset: &AVURLAsset,
) -> Result<objc2::rc::Retained<objc2_av_foundation::AVAssetTrack>> {
    let visual = unsafe { AVMediaCharacteristicVisual }
        .ok_or_else(|| invalid("visual media type unavailable"))?;
    unsafe { asset.tracks() }
        .to_vec()
        .into_iter()
        .find(|track| unsafe { track.hasMediaCharacteristic(visual) })
        .ok_or_else(|| invalid("output has no video track"))
}

fn frame_size(sample: &NativeSampleBuffer) -> Result<FrameSize> {
    let pixel = unsafe { sample.image_buffer() }
        .ok_or_else(|| invalid("decoded frame has no pixel buffer"))?;
    Ok(FrameSize {
        width: objc2_core_video::CVPixelBufferGetWidth(&pixel) as u32,
        height: objc2_core_video::CVPixelBufferGetHeight(&pixel) as u32,
    })
}

fn reader_error(reader: &AVAssetReader) -> AppError {
    unsafe { reader.error() }.map_or_else(
        || invalid("could not decode output"),
        |error| AppError::Encoder(error.localizedDescription().to_string()),
    )
}

fn invalid(reason: &str) -> AppError {
    AppError::InvalidConfig(format!("Secure Share decoded residual check {reason}"))
}
