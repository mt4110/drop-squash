use dropsquash_core::{
    AxObservationKind, CaptureFrameMetadata, CaptureRect, FrameSize, FrameStatus, PixelRect,
};

use dropsquash_core::MaskPolicy;

use super::ax_probe::{requires_strict_guard, strict_guard, time_range};

#[test]
fn strict_guard_covers_the_entire_capture_target() {
    let guard = strict_guard(FrameSize {
        width: 760,
        height: 392,
    });
    assert_eq!(guard.kind, AxObservationKind::UnknownClientArea);
    assert_eq!(
        guard.rect,
        PixelRect {
            x: 0,
            y: 0,
            width: 760,
            height: 392,
        }
    );
}

#[test]
fn strict_guard_is_not_added_to_smart_mask_recording() {
    assert!(requires_strict_guard(MaskPolicy::StrictReveal));
    assert!(!requires_strict_guard(MaskPolicy::SmartMask));
}

#[test]
fn accessibility_time_range_includes_the_final_capture_frame() {
    let range = time_range(&[frame(10), frame(20)]).unwrap();

    assert!(range.contains(10));
    assert!(range.contains(20));
}

fn frame(presentation_time_ns: u64) -> CaptureFrameMetadata {
    CaptureFrameMetadata {
        frame_index: 0,
        presentation_time_ns,
        frame_status: FrameStatus::Complete,
        content_rect: CaptureRect {
            x: 0,
            y: 0,
            width: 1,
            height: 1,
        },
        bounding_rect: CaptureRect {
            x: 0,
            y: 0,
            width: 1,
            height: 1,
        },
        scale_factor: 1.0,
        content_scale: 1.0,
    }
}
