use async_trait::async_trait;
use dropsquash_core::{AppError, EncodeJob, EncodeResult, OutputSize, Profile, Result};
use std::sync::Arc;

use crate::{verify_output, EncodeProgressReporter, EncoderBackend, EncoderCapabilities};

#[cfg(target_os = "macos")]
use std::{
    sync::mpsc::{sync_channel, RecvTimeoutError},
    time::Duration,
};

#[cfg(target_os = "macos")]
use block2::RcBlock;
#[cfg(target_os = "macos")]
use objc2::rc::autoreleasepool;
#[cfg(target_os = "macos")]
use objc2_av_foundation::{
    AVAssetExportPreset1280x720, AVAssetExportPreset640x480, AVAssetExportPreset960x540,
    AVAssetExportSession, AVAssetExportSessionStatus, AVFileTypeMPEG4, AVURLAsset,
};
#[cfg(target_os = "macos")]
use objc2_foundation::{NSString, NSURL};

#[derive(Debug, Clone, Default)]
pub struct VideoToolboxEncoder;

#[async_trait]
impl EncoderBackend for VideoToolboxEncoder {
    fn name(&self) -> &'static str {
        "videotoolbox"
    }

    fn probe_capabilities(&self) -> Result<EncoderCapabilities> {
        #[cfg(target_os = "macos")]
        {
            Ok(EncoderCapabilities {
                backend_name: self.name().to_string(),
                available: true,
                hardware_acceleration: false,
                supports_h264: true,
                supports_hevc: false,
                supports_metadata_strip: false,
                input_extensions: vec!["mov".to_string(), "mp4".to_string(), "m4v".to_string()],
            })
        }

        #[cfg(not(target_os = "macos"))]
        Ok(EncoderCapabilities {
            backend_name: self.name().to_string(),
            available: false,
            hardware_acceleration: false,
            supports_h264: false,
            supports_hevc: false,
            supports_metadata_strip: false,
            input_extensions: Vec::new(),
        })
    }

    async fn encode(&self, job: EncodeJob) -> Result<EncodeResult> {
        #[cfg(target_os = "macos")]
        {
            return autoreleasepool(|_| encode_with_avfoundation(job, None));
        }

        #[cfg(not(target_os = "macos"))]
        Err(AppError::Encoder(
            "VideoToolbox backend is not implemented in Phase 0".to_string(),
        ))
    }

    async fn encode_with_progress(
        &self,
        job: EncodeJob,
        reporter: Arc<dyn EncodeProgressReporter>,
    ) -> Result<EncodeResult> {
        #[cfg(target_os = "macos")]
        {
            return autoreleasepool(|_| encode_with_avfoundation(job, Some(reporter.as_ref())));
        }

        #[cfg(not(target_os = "macos"))]
        {
            reporter.report(0.0);
            let result = self.encode(job).await;
            if result.is_ok() {
                reporter.report(1.0);
            }
            result
        }
    }
}

#[cfg(target_os = "macos")]
fn encode_with_avfoundation(
    job: EncodeJob,
    reporter: Option<&dyn EncodeProgressReporter>,
) -> Result<EncodeResult> {
    if !job.input_path.is_file() {
        return Err(AppError::FileNotFound(job.input_path));
    }

    if job.profile == Profile::Privacy {
        return Err(AppError::Encoder(
            "Privacy profile requires the dedicated metadata and audio pipeline".to_string(),
        ));
    }

    std::fs::create_dir_all(&job.output_dir)?;
    let output_path = output_path_for(&job)?;

    let temporary_directory = tempfile::Builder::new()
        .prefix(".dropsquash-")
        .tempdir_in(&job.output_dir)?;
    let temporary_output = temporary_directory.path().join("output.mp4");
    let input_url = NSURL::from_file_path(&job.input_path).ok_or_else(|| {
        AppError::Encoder(format!(
            "could not create a file URL for {}",
            job.input_path.display()
        ))
    })?;
    let output_url = NSURL::from_file_path(&temporary_output).ok_or_else(|| {
        AppError::Encoder(format!(
            "could not create a file URL for {}",
            temporary_output.display()
        ))
    })?;
    let asset = unsafe { AVURLAsset::URLAssetWithURL_options(&input_url, None) };
    let preset = preset_for(job.profile, job.output_size)?;
    let export_session =
        unsafe { AVAssetExportSession::exportSessionWithAsset_presetName(&asset, preset) }
            .ok_or_else(|| {
                AppError::Encoder("no compatible Apple export preset was found".to_string())
            })?;
    let output_file_type = unsafe { AVFileTypeMPEG4 }.ok_or_else(|| {
        AppError::Encoder("MPEG-4 output is unavailable on this system".to_string())
    })?;

    unsafe {
        export_session.setOutputFileType(Some(output_file_type));
        export_session.setOutputURL(Some(&output_url));
        export_session.setShouldOptimizeForNetworkUse(true);
        export_session.setAllowsParallelizedExport(true);
    }

    let (completion_sender, completion_receiver) = sync_channel(1);
    let completion = RcBlock::new(move || {
        let _ = completion_sender.send(());
    });
    unsafe {
        export_session.exportAsynchronouslyWithCompletionHandler(&completion);
    }
    loop {
        match completion_receiver.recv_timeout(Duration::from_millis(100)) {
            Ok(()) => break,
            Err(RecvTimeoutError::Timeout) => {
                if let Some(reporter) = reporter {
                    reporter.report(unsafe { export_session.progress() }.clamp(0.0, 1.0));
                }
            }
            Err(RecvTimeoutError::Disconnected) => {
                return Err(AppError::Encoder(
                    "Apple export completed without a completion signal".to_string(),
                ));
            }
        }
    }

    let status = unsafe { export_session.status() };
    if status != AVAssetExportSessionStatus::Completed {
        let message = unsafe { export_session.error() }
            .map(|error| error.localizedDescription().to_string())
            .unwrap_or_else(|| format!("Apple export ended with status {}", status.0));
        return Err(AppError::Encoder(message));
    }

    let verification = verify_output(&job.input_path, &temporary_output)?;
    if !verification.is_smaller_than_original {
        return Err(AppError::Encoder(format!(
            "native export was not smaller than the original ({} bytes -> {} bytes)",
            verification.original_bytes, verification.output_bytes
        )));
    }

    std::fs::hard_link(&temporary_output, &output_path)?;
    if let Some(reporter) = reporter {
        reporter.report(1.0);
    }

    Ok(EncodeResult {
        input_path: job.input_path,
        output_path,
        profile: job.profile,
        original_bytes: verification.original_bytes,
        output_bytes: verification.output_bytes,
        success: true,
        error_message: None,
    })
}

#[cfg(target_os = "macos")]
fn output_path_for(job: &EncodeJob) -> Result<std::path::PathBuf> {
    let stem = job
        .input_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("recording");
    let primary = job.output_dir.join(format!("{stem}.squashed.mp4"));
    if !primary.try_exists()? {
        return Ok(primary);
    }

    for suffix in 2u32.. {
        let candidate = job.output_dir.join(format!("{stem}.squashed-{suffix}.mp4"));
        if !candidate.try_exists()? {
            return Ok(candidate);
        }
    }

    unreachable!("unbounded output suffix range must contain an available path")
}

#[cfg(target_os = "macos")]
fn preset_for(profile: Profile, output_size: OutputSize) -> Result<&'static NSString> {
    match output_size {
        OutputSize::P1080 => Ok(unsafe { objc2_av_foundation::AVAssetExportPreset1920x1080 }),
        OutputSize::P720 => Ok(unsafe { AVAssetExportPreset1280x720 }),
        OutputSize::P480 => Ok(unsafe { AVAssetExportPreset640x480 }),
        OutputSize::Auto => match profile {
            Profile::Auto
            | Profile::Slack
            | Profile::Chatwork
            | Profile::Line
            | Profile::WhatsApp => Ok(unsafe { AVAssetExportPreset960x540 }),
            Profile::Teams | Profile::Discord | Profile::Docs | Profile::Archive => {
                Ok(unsafe { AVAssetExportPreset1280x720 })
            }
            Profile::Privacy => Err(AppError::Encoder(
                "Privacy profile requires the dedicated metadata and audio pipeline".to_string(),
            )),
        },
    }
}

#[cfg(all(test, target_os = "macos"))]
mod tests {
    use super::*;
    use dropsquash_core::SourcePolicy;

    #[test]
    fn avfoundation_backend_reports_available() {
        let capabilities = VideoToolboxEncoder.probe_capabilities().unwrap();

        assert!(capabilities.available);
        assert!(capabilities.supports_h264);
        assert!(!capabilities.hardware_acceleration);
    }

    #[test]
    fn privacy_profile_does_not_select_a_lossy_preset() {
        assert!(preset_for(Profile::Privacy, OutputSize::Auto).is_err());
    }

    #[test]
    fn explicit_output_size_selects_a_native_preset() {
        assert!(preset_for(Profile::Auto, OutputSize::P1080).is_ok());
        assert!(preset_for(Profile::Auto, OutputSize::P720).is_ok());
        assert!(preset_for(Profile::Auto, OutputSize::P480).is_ok());
    }

    #[test]
    fn preserves_existing_output_by_selecting_a_numbered_path() {
        let directory = tempfile::tempdir().unwrap();
        let job = EncodeJob {
            input_path: directory.path().join("recording.mov"),
            output_dir: directory.path().to_path_buf(),
            profile: Profile::Auto,
            output_size: OutputSize::Auto,
            source_policy: SourcePolicy::Ask,
        };
        std::fs::write(directory.path().join("recording.squashed.mp4"), b"existing").unwrap();

        assert_eq!(
            output_path_for(&job).unwrap(),
            directory.path().join("recording.squashed-2.mp4")
        );
    }
}
