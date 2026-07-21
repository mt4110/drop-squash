use super::refined_accessibility;
use crate::{
    AxObservation, AxObservationKind, Confidence, PixelRect, TimeRangeNs, VisionObservation,
    VisionObservationKind,
};

#[test]
fn removes_an_ax_text_container_when_vision_finds_multiple_lines_inside_it() {
    let result = refined_accessibility(vec![ax(0, 0, 400, 100)], &[vision(20, 20), vision(20, 60)]);
    assert!(result.is_empty());
}

#[test]
fn retains_ax_text_when_vision_only_finds_one_candidate() {
    let result = refined_accessibility(vec![ax(0, 0, 400, 100)], &[vision(20, 20)]);
    assert_eq!(result.len(), 1);
}

#[test]
fn retains_non_text_ax_observations() {
    let mut observation = ax(0, 0, 400, 100);
    observation.kind = AxObservationKind::UnknownClientArea;
    let result = refined_accessibility(vec![observation], &[vision(20, 20), vision(20, 60)]);
    assert_eq!(result.len(), 1);
}

fn ax(x: u32, y: u32, width: u32, height: u32) -> AxObservation {
    AxObservation {
        rect: PixelRect {
            x,
            y,
            width,
            height,
        },
        time_range: time(),
        kind: AxObservationKind::TextElement,
        confidence: Confidence::CERTAIN,
    }
}

fn vision(x: u32, y: u32) -> VisionObservation {
    VisionObservation {
        rect: PixelRect {
            x,
            y,
            width: 120,
            height: 16,
        },
        time_range: time(),
        kind: VisionObservationKind::TextRecognition,
        confidence: Confidence::CERTAIN,
    }
}

fn time() -> TimeRangeNs {
    TimeRangeNs {
        start_ns: 0,
        end_ns: 1,
    }
}
