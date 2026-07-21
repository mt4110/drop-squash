use dropsquash_core::{AppError, PixelRect, Result};
use objc2::rc::Retained;
use objc2_av_foundation::{
    AVAssetWriter, AVAssetWriterInput, AVAssetWriterInputPixelBufferAdaptor, AVFileTypeMPEG4,
    AVMediaTypeVideo, AVVideoCodecKey, AVVideoCodecTypeH264,
};
use objc2_core_media::CMSampleBuffer;
use objc2_foundation::{NSCopying, NSDictionary, NSString, NSURL};
use std::path::PathBuf;
use std::time::Duration;

mod error;
use error::{missing_codec, missing_file_type, missing_format, writer_error, writer_status};
#[derive(Debug)]
pub(super) struct SckRecordingWriter {
    path: PathBuf,
    writer: Retained<AVAssetWriter>,
    input: Retained<AVAssetWriterInput>,
    adaptor: Retained<AVAssetWriterInputPixelBufferAdaptor>,
}

impl SckRecordingWriter {
    pub(super) fn start(path: PathBuf, sample: &CMSampleBuffer) -> Result<Self> {
        if path.exists() {
            return Err(AppError::InvalidConfig(
                "Secure Share recording output already exists".to_string(),
            ));
        }
        let format = unsafe { sample.format_description() }.ok_or_else(missing_format)?;
        let url = NSURL::from_file_path(&path).ok_or_else(|| {
            AppError::InvalidConfig("Secure Share recording output path is invalid".to_string())
        })?;
        let file_type = unsafe { AVFileTypeMPEG4 }.ok_or_else(missing_file_type)?;
        let writer = unsafe { AVAssetWriter::assetWriterWithURL_fileType_error(&url, file_type) }
            .map_err(|error| writer_error("could not create output", &error))?;
        let settings = writer_settings()?;
        let input = unsafe {
            AVAssetWriterInput::assetWriterInputWithMediaType_outputSettings_sourceFormatHint(
                AVMediaTypeVideo.unwrap(),
                Some(&settings),
                Some(&format),
            )
        };
        unsafe {
            input.setExpectsMediaDataInRealTime(true);
            writer.addInput(&input);
        }
        let adaptor = unsafe {
            AVAssetWriterInputPixelBufferAdaptor::assetWriterInputPixelBufferAdaptorWithAssetWriterInput_sourcePixelBufferAttributes(&input, None)
        };
        if !unsafe { writer.startWriting() } {
            return Err(writer_status("could not start", &writer));
        }
        unsafe { writer.startSessionAtSourceTime(sample.presentation_time_stamp()) };
        Ok(Self {
            path,
            writer,
            input,
            adaptor,
        })
    }

    pub(super) fn append(
        &self,
        sample: &CMSampleBuffer,
        regions: &[PixelRect],
    ) -> Result<super::live_mask::FrameMaskProof> {
        for _ in 0..20 {
            if unsafe { self.input.isReadyForMoreMediaData() } {
                let frame = super::live_mask::copied_blackened(sample, regions)?;
                return unsafe {
                    self.adaptor.appendPixelBuffer_withPresentationTime(
                        &frame.pixel,
                        sample.presentation_time_stamp(),
                    )
                }
                .then_some(frame.proof)
                .ok_or_else(|| writer_status("could not append", &self.writer));
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        Err(writer_status("input stalled", &self.writer))
    }

    pub(super) fn finish(self) -> Result<PathBuf> {
        unsafe { self.input.markAsFinished() };
        #[allow(deprecated)]
        if unsafe { self.writer.finishWriting() } {
            return self.path.is_file().then_some(self.path).ok_or_else(|| {
                AppError::Encoder("Secure Share recording output is missing".to_string())
            });
        }
        Err(writer_status("could not finish", &self.writer))
    }

    pub(super) fn discard(self) {
        unsafe { self.writer.cancelWriting() };
        let _ = std::fs::remove_file(self.path);
    }
}

fn writer_settings() -> Result<Retained<NSDictionary<NSString, objc2::runtime::AnyObject>>> {
    let codec = unsafe { AVVideoCodecTypeH264 }.ok_or_else(missing_codec)?;
    let key = unsafe { AVVideoCodecKey }.ok_or_else(missing_codec)?;
    Ok(NSDictionary::from_retained_objects(
        &[key],
        &[codec.copy().into()],
    ))
}
