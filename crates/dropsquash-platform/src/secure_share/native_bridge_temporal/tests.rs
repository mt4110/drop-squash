use dropsquash_core::{CaptureFrameMetadata, CaptureRect, FrameSize, FrameStatus, PixelRect};

use super::super::temporal_observations_from_native;
use crate::NativeTemporalObservation;

#[test]
fn maps_native_change_to_its_frame_time() {
    let result = temporal_observations_from_native(
        vec![NativeTemporalObservation {
            frame_index: 0,
            x: 0.25,
            y: 0.5,
            width: 0.5,
            height: 0.25,
        }],
        &[frame()],
        FrameSize {
            width: 400,
            height: 200,
        },
    )
    .unwrap();

    assert_eq!(
        result[0].rect,
        PixelRect {
            x: 100,
            y: 100,
            width: 200,
            height: 50
        }
    );
    assert_eq!(result[0].time_range.start_ns, 42);
}

fn frame() -> CaptureFrameMetadata {
    CaptureFrameMetadata {
        frame_index: 0,
        presentation_time_ns: 42,
        frame_status: FrameStatus::Complete,
        content_rect: CaptureRect {
            x: 0,
            y: 0,
            width: 4,
            height: 4,
        },
        bounding_rect: CaptureRect {
            x: 0,
            y: 0,
            width: 4,
            height: 4,
        },
        scale_factor: 1.0,
        content_scale: 1.0,
    }
}
