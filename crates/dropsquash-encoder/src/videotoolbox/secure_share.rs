use dropsquash_core::{EncodeJob, EncodeResult, Result, SecureShareOptions};
use objc2::rc::Retained;
use objc2_av_foundation::{
    AVAssetReader, AVAssetReaderTrackOutput, AVAssetTrack, AVAssetWriter, AVAssetWriterInput,
    AVFileTypeMPEG4, AVMediaCharacteristicVisual, AVURLAsset,
};
use objc2_core_media::CMFormatDescription;
use objc2_foundation::NSError;

use crate::verify_output;

use super::encode::verification_error;
use super::finalize::finalize_verified_output;
use super::paths::output_path_for;
use super::secure_share_flow::run_masked_export;
use super::secure_share_settings::{reader_output_settings, writer_output_settings};
use super::session::file_url;

pub(super) fn encode_with_reader_writer(job: &EncodeJob) -> Result<EncodeResult> {
    std::fs::create_dir_all(&job.output_dir)?;
    let output_path = output_path_for(job)?;
    let temp = tempfile::Builder::new()
        .prefix(".dropsquash-secure-share-")
        .tempdir_in(&job.output_dir)?;
    let temp_output = temp.path().join("output.mp4");
    let pipeline = bootstrap(job, &temp_output)?;
    run_masked_export(&pipeline, required_options(job)?)?;
    let verification = verify_output(&job.input_path, &temp_output)?;
    if !verification.is_valid_output {
        return Err(verification_error(Some(verification)));
    }
    finalize_verified_output(&temp_output, &output_path)?;
    Ok(EncodeResult {
        input_path: job.input_path.clone(),
        output_path,
        profile: job.profile,
        original_bytes: verification.original_bytes,
        output_bytes: verification.output_bytes,
        success: true,
        error_message: None,
    })
}

fn required_options(job: &EncodeJob) -> Result<&SecureShareOptions> {
    Ok(job
        .secure_share
        .as_ref()
        .expect("secure-share jobs are gated by caller"))
}

fn bootstrap(job: &EncodeJob, temp_output: &std::path::Path) -> Result<PipelineBootstrap> {
    let input_url = file_url(&job.input_path)?;
    let output_url = file_url(temp_output)?;
    let asset = unsafe { AVURLAsset::URLAssetWithURL_options(&input_url, None) };
    let track = video_track(&asset)?;
    let reader_settings = reader_output_settings();
    let reader = unsafe { AVAssetReader::assetReaderWithAsset_error(&asset) }
        .map_err(|e| ns_error("Secure Share asset bootstrap failed", &e))?;
    let output = unsafe {
        AVAssetReaderTrackOutput::assetReaderTrackOutputWithTrack_outputSettings(
            &track,
            Some(&reader_settings),
        )
    };
    unsafe { reader.addOutput(&output) };
    let writer_settings = writer_output_settings()?;
    let format_hint = format_hint(&track)?;
    let file_type = unsafe { AVFileTypeMPEG4 }.unwrap();
    let writer =
        unsafe { AVAssetWriter::assetWriterWithURL_fileType_error(&output_url, file_type) }
            .map_err(|e| ns_error("Secure Share writer bootstrap failed", &e))?;
    let input = unsafe {
        AVAssetWriterInput::assetWriterInputWithMediaType_outputSettings_sourceFormatHint(
            &track.mediaType(),
            Some(&writer_settings),
            Some(&format_hint),
        )
    };
    unsafe { input.setExpectsMediaDataInRealTime(false) };
    unsafe { writer.addInput(&input) };
    Ok(PipelineBootstrap {
        reader,
        output,
        writer,
        input,
    })
}

fn video_track(asset: &AVURLAsset) -> Result<Retained<AVAssetTrack>> {
    let visual = unsafe { AVMediaCharacteristicVisual }.unwrap();
    unsafe { asset.tracks() }
        .to_vec()
        .into_iter()
        .find(|track| unsafe { track.hasMediaCharacteristic(visual) })
        .ok_or_else(|| {
            dropsquash_core::AppError::UnsupportedMedia(
                "Secure Share requires a video track".to_string(),
            )
        })
}

fn format_hint(track: &AVAssetTrack) -> Result<Retained<CMFormatDescription>> {
    let description = unsafe { track.formatDescriptions().firstObject() }.ok_or_else(|| {
        dropsquash_core::AppError::UnsupportedMedia(
            "Secure Share requires a video format description".to_string(),
        )
    })?;
    Ok(unsafe { Retained::cast_unchecked(description) })
}

pub(super) struct PipelineBootstrap {
    pub reader: Retained<AVAssetReader>,
    pub output: Retained<AVAssetReaderTrackOutput>,
    pub writer: Retained<AVAssetWriter>,
    pub input: Retained<AVAssetWriterInput>,
}

fn ns_error(prefix: &str, error: &NSError) -> dropsquash_core::AppError {
    dropsquash_core::AppError::Encoder(format!("{prefix}: {}", error.localizedDescription()))
}
