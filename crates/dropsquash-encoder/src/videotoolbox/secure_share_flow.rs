use std::time::Duration;

use dropsquash_core::{AppError, Result, SecureShareOptions};
use objc2_av_foundation::{
    AVAssetReader, AVAssetReaderStatus, AVAssetWriter, AVAssetWriterInput, AVAssetWriterStatus,
};

use super::pixel_buffer::{mask_sample_buffer, sample_frame_size};
use super::secure_share::PipelineBootstrap;

pub(super) fn run_masked_export(
    pipeline: &PipelineBootstrap,
    options: &SecureShareOptions,
) -> Result<()> {
    if !unsafe { pipeline.reader.startReading() } {
        return Err(reader_error(
            "Secure Share reader failed to start",
            &pipeline.reader,
        ));
    }
    if !unsafe { pipeline.writer.startWriting() } {
        return Err(writer_error(
            "Secure Share writer failed to start",
            &pipeline.writer,
        ));
    }
    let mut frame = 0u64;
    let mut started = false;
    loop {
        wait_until_ready(&pipeline.input, &pipeline.writer)?;
        let sample = match unsafe { pipeline.output.copyNextSampleBuffer() } {
            Some(sample) => sample,
            None => {
                finish_reading(&pipeline.reader)?;
                break;
            }
        };
        if unsafe { sample.num_samples() } == 0 {
            continue;
        }
        if !started {
            unsafe {
                pipeline
                    .writer
                    .startSessionAtSourceTime(sample.presentation_time_stamp())
            };
            started = true;
        }
        let frame_options =
            crate::secure_share::options_for_frame(options, frame, sample_frame_size(&sample)?)?;
        mask_sample_buffer(&sample, &frame_options, frame)?;
        if !unsafe { pipeline.input.appendSampleBuffer(&sample) } {
            return Err(writer_error(
                "Secure Share frame append failed",
                &pipeline.writer,
            ));
        }
        frame += 1;
    }
    if !started {
        return Err(AppError::UnsupportedMedia(
            "Secure Share requires at least one readable video frame".to_string(),
        ));
    }
    unsafe { pipeline.input.markAsFinished() };
    #[allow(deprecated)]
    if unsafe { pipeline.writer.finishWriting() } {
        return Ok(());
    }
    Err(writer_error(
        "Secure Share writer failed to finish",
        &pipeline.writer,
    ))
}

fn wait_until_ready(input: &AVAssetWriterInput, writer: &AVAssetWriter) -> Result<()> {
    while !unsafe { input.isReadyForMoreMediaData() } {
        match unsafe { writer.status() } {
            AVAssetWriterStatus::Cancelled => return Err(AppError::Cancelled),
            AVAssetWriterStatus::Failed => {
                return Err(writer_error("Secure Share writer stalled", writer));
            }
            _ => std::thread::sleep(Duration::from_millis(10)),
        }
    }
    Ok(())
}

fn finish_reading(reader: &AVAssetReader) -> Result<()> {
    match unsafe { reader.status() } {
        AVAssetReaderStatus::Completed => Ok(()),
        AVAssetReaderStatus::Cancelled => Err(AppError::Cancelled),
        AVAssetReaderStatus::Failed => Err(reader_error("Secure Share reader failed", reader)),
        _ => Err(AppError::Encoder(
            "Secure Share reader ended before reporting completion".to_string(),
        )),
    }
}

fn reader_error(prefix: &str, reader: &AVAssetReader) -> AppError {
    unsafe { reader.error() }.map_or_else(
        || AppError::Encoder(prefix.to_string()),
        |error| AppError::Encoder(format!("{prefix}: {}", error.localizedDescription())),
    )
}

fn writer_error(prefix: &str, writer: &AVAssetWriter) -> AppError {
    unsafe { writer.error() }.map_or_else(
        || AppError::Encoder(prefix.to_string()),
        |error| AppError::Encoder(format!("{prefix}: {}", error.localizedDescription())),
    )
}
