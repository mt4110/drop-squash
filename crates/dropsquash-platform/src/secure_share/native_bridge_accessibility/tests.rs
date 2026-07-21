use super::{
    accessibility_observations_from_native, native_observation, NativeAccessibilityObservation,
    SckCaptureTarget,
};
use crate::SckCaptureTargetKind;
use dropsquash_core::{
    AxObservationKind, CaptureFrameMetadata, CaptureRect, FrameSize, FrameStatus, PixelRect,
};

#[test]
fn rejects_an_unknown_native_accessibility_kind() {
    let value = NativeAccessibilityObservation {
        frame_index: 0,
        kind: 0,
        x: 0,
        y: 0,
        width: 1,
        height: 1,
    };
    assert!(native_observation(value, &target(), size(), &[frame()]).is_err());
}

#[test]
fn accepts_a_text_element_without_text_content() {
    let value = NativeAccessibilityObservation {
        frame_index: 0,
        kind: 1,
        x: 10,
        y: 20,
        width: 30,
        height: 40,
    };
    let observation =
        native_observation(value, &target(), size(), &[frame()]).expect("text geometry is valid");
    assert_eq!(observation.rect.x, 10);
    assert_eq!(observation.rect.width, 30);
}

#[test]
fn keeps_focused_text_geometry_distinct_from_tree_text() {
    let value = NativeAccessibilityObservation {
        frame_index: 0,
        kind: 2,
        x: 10,
        y: 20,
        width: 30,
        height: 40,
    };
    let observation = native_observation(value, &target(), size(), &[frame()])
        .expect("focused text geometry is valid");
    assert_eq!(observation.kind, AxObservationKind::FocusedTextElement);
}

#[test]
fn maps_native_global_coordinates_to_retina_capture_pixels() {
    let value = NativeAccessibilityObservation {
        frame_index: 0,
        kind: 1,
        x: 120,
        y: 240,
        width: 20,
        height: 10,
    };
    let result = accessibility_observations_from_native(vec![value], &target(), size(), &[frame()])
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
fn accepts_negative_global_coordinates_on_an_external_display() {
    let value = NativeAccessibilityObservation {
        frame_index: 0,
        kind: 1,
        x: -1800,
        y: 120,
        width: 20,
        height: 10,
    };
    let target = SckCaptureTarget {
        frame: CaptureRect {
            x: -1920,
            y: 100,
            width: 400,
            height: 200,
        },
        ..target()
    };
    let result = accessibility_observations_from_native(vec![value], &target, size(), &[frame()])
        .expect("external-display geometry must map");
    assert_eq!(
        result[0].rect,
        PixelRect {
            x: 120,
            y: 20,
            width: 20,
            height: 10
        }
    );
}

#[test]
fn rejects_geometry_outside_the_selected_external_window() {
    let value = NativeAccessibilityObservation {
        frame_index: 0,
        kind: 1,
        x: -1921,
        y: 120,
        width: 20,
        height: 10,
    };
    let target = SckCaptureTarget {
        frame: CaptureRect {
            x: -1920,
            y: 100,
            width: 400,
            height: 200,
        },
        ..target()
    };
    assert!(
        accessibility_observations_from_native(vec![value], &target, size(), &[frame()]).is_err()
    );
}

#[test]
fn attaches_geometry_to_its_native_capture_frame() {
    let value = NativeAccessibilityObservation {
        frame_index: 1,
        kind: 1,
        x: 120,
        y: 240,
        width: 20,
        height: 10,
    };
    let frames = [
        frame(),
        CaptureFrameMetadata {
            presentation_time_ns: 99,
            ..frame()
        },
    ];
    let result = accessibility_observations_from_native(vec![value], &target(), size(), &frames)
        .expect("frame-indexed geometry must map");
    assert_eq!(result[0].time_range.start_ns, 99);
    assert_eq!(result[0].time_range.end_ns, 100);
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

fn size() -> FrameSize {
    FrameSize {
        width: 400,
        height: 200,
    }
}

fn frame() -> CaptureFrameMetadata {
    CaptureFrameMetadata {
        frame_index: 0,
        presentation_time_ns: 1,
        frame_status: FrameStatus::Complete,
        content_rect: CaptureRect {
            x: 0,
            y: 0,
            width: 400,
            height: 200,
        },
        bounding_rect: CaptureRect {
            x: 0,
            y: 0,
            width: 400,
            height: 200,
        },
        scale_factor: 2.0,
        content_scale: 1.0,
    }
}
