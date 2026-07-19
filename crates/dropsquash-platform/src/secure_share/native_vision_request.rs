use dropsquash_core::{AppError, Result};
use objc2::rc::Retained;
use objc2::ClassType;
use objc2_foundation::{ns_string, NSArray};

use super::{
    NativeVisionImageRequestHandler, NativeVisionRequest, NativeVisionTextRequest,
    NativeVisionTextShapeRequest,
};

#[derive(Debug)]
pub struct NativeVisionRequests {
    pub text: Retained<NativeVisionTextRequest>,
    pub text_shape: Retained<NativeVisionTextShapeRequest>,
}

pub fn make_text_request() -> Retained<NativeVisionTextRequest> {
    let request = NativeVisionTextRequest::new();
    request.setRecognitionLanguages(&recognition_languages());
    request.setUsesLanguageCorrection(false);
    request.setRecognitionLevel(objc2_vision::VNRequestTextRecognitionLevel::Accurate);
    request
}

pub fn make_text_shape_request() -> Retained<NativeVisionTextShapeRequest> {
    let request = unsafe { NativeVisionTextShapeRequest::new() };
    unsafe { request.setReportCharacterBoxes(false) };
    request
}

pub fn perform_text_requests(
    handler: &NativeVisionImageRequestHandler,
) -> Result<NativeVisionRequests> {
    let requests = NativeVisionRequests {
        text: make_text_request(),
        text_shape: make_text_shape_request(),
    };
    let scheduled = NSArray::<NativeVisionRequest>::from_slice(&[
        requests.text.as_super().as_super(),
        requests.text_shape.as_super().as_super(),
    ]);
    handler.performRequests_error(&scheduled).map_err(|error| {
        AppError::InvalidConfig(format!("Secure Share Vision request failed: {error:?}"))
    })?;
    Ok(requests)
}

fn recognition_languages() -> Retained<NSArray<objc2_foundation::NSString>> {
    NSArray::from_slice(&[ns_string!("ja-JP"), ns_string!("en-US")])
}
