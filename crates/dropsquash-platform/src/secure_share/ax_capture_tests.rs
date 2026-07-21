use dropsquash_core::{
    AxObservation, AxObservationKind, CaptureRect, Confidence, FrameSize, PixelRect, TimeRangeNs,
};

use super::{ax_capture::map_to_capture, SckCaptureTarget, SckCaptureTargetKind};

#[test]
fn maps_global_ax_coordinates_to_capture_pixels() {
    let result = map_to_capture(vec![observation(120, 240, 20, 10)], &target(), frame_size())
        .expect("inside target must map");
    assert_eq!(
        result[0].rect,
        PixelRect {
            x: 40,
            y: 80,
            width: 40,
            height: 20
        }
    );
}

#[test]
fn excludes_ax_coordinate_outside_capture_target() {
    let result = map_to_capture(vec![observation(90, 240, 20, 10)], &target(), frame_size());
    assert!(result.unwrap().is_empty());
}

fn target() -> SckCaptureTarget {
    SckCaptureTarget {
        kind: SckCaptureTargetKind::Window,
        id: 7,
        frame: CaptureRect {
            x: 100,
            y: 200,
            width: 200,
            height: 100,
        },
        owner_pid: Some(1),
    }
}

fn frame_size() -> FrameSize {
    FrameSize {
        width: 400,
        height: 200,
    }
}

fn observation(x: u32, y: u32, width: u32, height: u32) -> AxObservation {
    AxObservation {
        rect: PixelRect {
            x,
            y,
            width,
            height,
        },
        time_range: TimeRangeNs {
            start_ns: 0,
            end_ns: u64::MAX,
        },
        kind: AxObservationKind::UnknownClientArea,
        confidence: Confidence::CERTAIN,
    }
}
