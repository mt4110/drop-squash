use dropsquash_core::{AppError, EncodeJob, OutputSize, Profile, Result};
use dropsquash_media::probe;
use dropsquash_profiles::resolve_profile;
use objc2_av_foundation::{
    AVAssetExportPreset1280x720, AVAssetExportPreset640x480, AVAssetExportPreset960x540,
    AVAssetExportPresetLowQuality, AVAssetExportPresetMediumQuality,
};
use objc2_foundation::NSString;

pub(crate) fn presets_for(job: &EncodeJob) -> Result<Vec<&'static NSString>> {
    match job.output_size {
        OutputSize::P1080 => Ok(max_size_presets(OutputSize::P1080)),
        OutputSize::P720 => Ok(max_size_presets(OutputSize::P720)),
        OutputSize::P480 => Ok(max_size_presets(OutputSize::P480)),
        OutputSize::Auto => auto_presets(job),
    }
}

fn auto_presets(job: &EncodeJob) -> Result<Vec<&'static NSString>> {
    let media = probe(&job.input_path)?;
    match resolve_profile(job.profile, &media) {
        Profile::Auto | Profile::Slack | Profile::Chatwork | Profile::Line | Profile::WhatsApp => {
            Ok(chat_chain())
        }
        Profile::Teams | Profile::Discord | Profile::Docs | Profile::Archive => {
            Ok(standard_chain())
        }
        Profile::Privacy => Err(AppError::Encoder(
            "Privacy profile requires the dedicated metadata and audio pipeline".to_string(),
        )),
    }
}

fn max_size_presets(size: OutputSize) -> Vec<&'static NSString> {
    match size {
        OutputSize::P1080 => {
            let mut presets = vec![unsafe { objc2_av_foundation::AVAssetExportPreset1920x1080 }];
            presets.extend(standard_chain());
            presets
        }
        OutputSize::P720 => standard_chain(),
        OutputSize::P480 => fallback_quality_chain(),
        OutputSize::Auto => Vec::new(),
    }
}

fn standard_chain() -> Vec<&'static NSString> {
    let mut presets = vec![unsafe { AVAssetExportPreset1280x720 }, unsafe {
        AVAssetExportPreset960x540
    }];
    presets.extend(fallback_quality_chain());
    presets
}

fn chat_chain() -> Vec<&'static NSString> {
    let mut presets = vec![unsafe { AVAssetExportPreset960x540 }];
    presets.extend(fallback_quality_chain());
    presets
}

fn fallback_quality_chain() -> Vec<&'static NSString> {
    vec![
        unsafe { AVAssetExportPreset640x480 },
        unsafe { AVAssetExportPresetMediumQuality },
        unsafe { AVAssetExportPresetLowQuality },
    ]
}
