use dropsquash_core::{
    Confidence, FrameSize, TimeRangeNs, VisionNormalizedRect, VisionObservation,
    VisionObservationKind,
};
use objc2_core_foundation::CGRect;
use objc2_foundation::NSArray;

use super::{NativeVisionTextRequest, NativeVisionTextShapeRequest};

pub fn text_observations(
    request: &NativeVisionTextRequest,
    frame_size: FrameSize,
    time_range: TimeRangeNs,
) -> Vec<VisionObservation> {
    let Some(results) = request.results() else {
        return Vec::new();
    };
    let mut observations = Vec::new();
    for index in 0..results.count() {
        let result = results.objectAtIndex(index);
        let rect = unsafe { result.boundingBox() };
        let confidence = unsafe { result.confidence() };
        if let Some(observation) = observation(
            rect,
            frame_size,
            time_range,
            VisionObservationKind::TextRecognition,
            confidence,
        ) {
            observations.push(observation);
        }
    }
    observations
}

pub fn text_shape_observations(
    request: &NativeVisionTextShapeRequest,
    frame_size: FrameSize,
    time_range: TimeRangeNs,
) -> Vec<VisionObservation> {
    let Some(results) = (unsafe { request.results() }) else {
        return Vec::new();
    };
    observations_from_text_shapes(&results, frame_size, time_range)
}

pub fn observations_from_text_shapes(
    results: &NSArray<objc2_vision::VNTextObservation>,
    frame_size: FrameSize,
    time_range: TimeRangeNs,
) -> Vec<VisionObservation> {
    let mut observations = Vec::new();
    for index in 0..results.count() {
        let result = results.objectAtIndex(index);
        let rect = unsafe { result.boundingBox() };
        let confidence = unsafe { result.confidence() };
        if let Some(observation) = observation(
            rect,
            frame_size,
            time_range,
            VisionObservationKind::TextRectangle,
            confidence,
        ) {
            observations.push(observation);
        }
    }
    observations
}

fn observation(
    rect: CGRect,
    frame_size: FrameSize,
    time_range: TimeRangeNs,
    kind: VisionObservationKind,
    detection: f32,
) -> Option<VisionObservation> {
    normalized_rect(rect).to_observation(
        frame_size,
        time_range,
        kind,
        Confidence {
            detection,
            transform: 1.0,
            policy: 1.0,
        },
    )
}

fn normalized_rect(rect: CGRect) -> VisionNormalizedRect {
    VisionNormalizedRect {
        x: rect.origin.x,
        y: rect.origin.y,
        width: rect.size.width,
        height: rect.size.height,
    }
}
