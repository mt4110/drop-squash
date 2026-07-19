use dropsquash_core::{AppError, CaptureFrameMetadata, FrameSize, Result};

use super::FrameMetadataProvider;

#[derive(Debug, Clone, Default)]
pub struct NativeFrameMetadataProvider;

#[cfg(target_os = "macos")]
pub type NativeFrameInfoKey = objc2_screen_capture_kit::SCStreamFrameInfo;

#[cfg(not(target_os = "macos"))]
pub type NativeFrameInfoKey = str;

impl FrameMetadataProvider for NativeFrameMetadataProvider {
    fn frame_size(&self) -> Result<FrameSize> {
        native_frame_metadata_error()
    }

    fn frames(&self) -> Result<Vec<CaptureFrameMetadata>> {
        native_frame_metadata_error()
    }
}

#[cfg(target_os = "macos")]
fn native_frame_metadata_error<T>() -> Result<T> {
    Err(AppError::UnsupportedMedia(
        "Secure Share probe ScreenCaptureKit frame metadata provider is not implemented yet"
            .to_string(),
    ))
}

#[cfg(not(target_os = "macos"))]
fn native_frame_metadata_error<T>() -> Result<T> {
    Err(AppError::UnsupportedMedia(
        "Secure Share frame metadata provider is macOS-only".to_string(),
    ))
}

pub fn screen_capture_kit_binding_name() -> &'static str {
    #[cfg(target_os = "macos")]
    {
        let _ = core::mem::size_of::<&NativeFrameInfoKey>();
    }
    "objc2-screen-capture-kit"
}
