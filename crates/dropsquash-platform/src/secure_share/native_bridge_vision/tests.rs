use dropsquash_core::{CaptureFrameMetadata, CaptureRect, FrameSize, FrameStatus};

use super::vision_observations_from_native;
use crate::NativeVisionObservation;

#[test]
fn transforms_native_vision_rectangles_without_text() {
    let values = vec![NativeVisionObservation {
        frame_index: 0,
        kind: 1,
        x: 0.1,
        y: 0.2,
        width: 0.3,
        height: 0.4,
        confidence: 0.9,
    }];
    let observation = vision_observations_from_native(
        values,
        &frames(),
        FrameSize {
            width: 100,
            height: 100,
        },
    )
    .unwrap()
    .pop()
    .unwrap();

    assert_eq!(observation.rect.x, 10);
    assert_eq!(observation.rect.y, 40);
    assert_eq!(observation.rect.width, 31);
    assert_eq!(observation.rect.height, 40);
}

#[test]
fn rejects_unknown_native_vision_kinds() {
    let result = vision_observations_from_native(
        vec![NativeVisionObservation {
            frame_index: 0,
            kind: 99,
            x: 0.1,
            y: 0.2,
            width: 0.3,
            height: 0.4,
            confidence: 0.9,
        }],
        &frames(),
        FrameSize {
            width: 100,
            height: 100,
        },
    );

    assert!(result.is_err());
}

fn frames() -> Vec<CaptureFrameMetadata> {
    vec![CaptureFrameMetadata {
        frame_index: 0,
        presentation_time_ns: 4,
        frame_status: FrameStatus::Complete,
        content_rect: CaptureRect {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
        },
        bounding_rect: CaptureRect {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
        },
        scale_factor: 1.0,
        content_scale: 1.0,
    }]
}
