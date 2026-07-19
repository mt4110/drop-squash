use dropsquash_core::{AppError, Result, VisionObservation};
use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::AnyThread;
use objc2_foundation::NSDictionary;

use super::{raw_sck_frame_info_from_sample_buffer, NativeSampleBuffer, VisionObservationProvider};

#[derive(Debug, Clone, Default)]
pub struct NativeVisionObservationProvider;

pub type NativeVisionImageOption = objc2_vision::VNImageOption;
pub type NativeVisionImageRequestHandler = objc2_vision::VNImageRequestHandler;
pub type NativeVisionRequest = objc2_vision::VNRequest;
pub type NativeVisionTextRequest = objc2_vision::VNRecognizeTextRequest;
pub type NativeVisionTextShapeRequest = objc2_vision::VNDetectTextRectanglesRequest;

impl VisionObservationProvider for NativeVisionObservationProvider {
    fn vision_observations(&self) -> Result<Vec<VisionObservation>> {
        Err(AppError::UnsupportedMedia(
            "Secure Share Vision observation requires captured frame pixel buffers".to_string(),
        ))
    }
}

pub fn sample_buffer_request_handler(
    sample_buffer: &NativeSampleBuffer,
) -> Result<Retained<NativeVisionImageRequestHandler>> {
    raw_sck_frame_info_from_sample_buffer(sample_buffer, 0)?;
    let options = NSDictionary::<NativeVisionImageOption, AnyObject>::dictionary();
    let handler = unsafe {
        NativeVisionImageRequestHandler::initWithCMSampleBuffer_options(
            NativeVisionImageRequestHandler::alloc(),
            sample_buffer,
            &options,
        )
    };
    Ok(handler)
}

pub fn vision_binding_name() -> &'static str {
    let _ = core::mem::size_of::<&NativeVisionImageRequestHandler>();
    let _ = core::mem::size_of::<&NativeVisionRequest>();
    let _ = core::mem::size_of::<&NativeVisionTextRequest>();
    let _ = core::mem::size_of::<&NativeVisionTextShapeRequest>();
    "objc2-vision"
}
