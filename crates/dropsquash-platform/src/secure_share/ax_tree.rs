use std::ffi::c_void;
use std::ptr::NonNull;

use dropsquash_core::{AxObservation, AxObservationKind, TimeRangeNs};
use objc2_application_services::AXUIElement;
use objc2_core_foundation::{CFArray, CFRetained, CFString};

use super::ax_observation::{copy_attribute, copy_ax_point, copy_ax_size};

const MAX_ACCESSIBILITY_DESCENT_DEPTH: u8 = 8;

pub(super) fn text_observations(
    window: &AXUIElement,
    time_range: TimeRangeNs,
) -> Vec<AxObservation> {
    let mut observations = Vec::new();
    visit(window, time_range, 0, &mut observations);
    observations
}

fn visit(
    element: &AXUIElement,
    time_range: TimeRangeNs,
    depth: u8,
    observations: &mut Vec<AxObservation>,
) {
    if is_text_element(element) {
        if let (Ok(point), Ok(size)) = (
            copy_ax_point(element, "AXPosition"),
            copy_ax_size(element, "AXSize"),
        ) {
            if let Ok(mut observation) =
                super::ax_rect::observation_from_point_size(point, size, time_range)
            {
                observation.kind = AxObservationKind::TextElement;
                observations.push(observation);
            }
        }
    }
    if depth == MAX_ACCESSIBILITY_DESCENT_DEPTH {
        return;
    }
    for child in children(element) {
        visit(&child, time_range, depth + 1, observations);
    }
}

fn is_text_element(element: &AXUIElement) -> bool {
    copy_attribute(element, "AXRole")
        .ok()
        .and_then(|value| value.downcast::<CFString>().ok())
        .is_some_and(|role| {
            matches!(
                role.to_string().as_str(),
                "AXStaticText" | "AXTextField" | "AXTextArea"
            )
        })
}

fn children(element: &AXUIElement) -> Vec<CFRetained<AXUIElement>> {
    let Ok(value) = copy_attribute(element, "AXChildren") else {
        return Vec::new();
    };
    let Ok(items) = value.downcast::<CFArray>() else {
        return Vec::new();
    };
    (0..items.count())
        .filter_map(|index| retain_child(unsafe { items.value_at_index(index) }))
        .collect()
}

fn retain_child(child: *const c_void) -> Option<CFRetained<AXUIElement>> {
    let child = NonNull::new(child.cast_mut())?.cast::<AXUIElement>();
    Some(unsafe { CFRetained::retain(child) })
}
