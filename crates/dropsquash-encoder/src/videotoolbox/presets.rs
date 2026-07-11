use dropsquash_core::{AppError, EncodeJob, OutputSize, Profile, Result};
use objc2_av_foundation::{
    AVAssetExportPreset1280x720, AVAssetExportPreset640x480, AVAssetExportPreset960x540,
};
use objc2_foundation::NSString;

pub(crate) fn preset_for(job: &EncodeJob) -> Result<&'static NSString> {
    match job.output_size {
        OutputSize::P1080 => Ok(unsafe { objc2_av_foundation::AVAssetExportPreset1920x1080 }),
        OutputSize::P720 => Ok(unsafe { AVAssetExportPreset1280x720 }),
        OutputSize::P480 => Ok(unsafe { AVAssetExportPreset640x480 }),
        OutputSize::Auto => auto_preset(job.profile),
    }
}

fn auto_preset(profile: Profile) -> Result<&'static NSString> {
    match profile {
        Profile::Auto | Profile::Slack | Profile::Chatwork | Profile::Line | Profile::WhatsApp => {
            Ok(unsafe { AVAssetExportPreset960x540 })
        }
        Profile::Teams | Profile::Discord | Profile::Docs | Profile::Archive => {
            Ok(unsafe { AVAssetExportPreset1280x720 })
        }
        Profile::Privacy => Err(AppError::Encoder(
            "Privacy profile requires the dedicated metadata and audio pipeline".to_string(),
        )),
    }
}
