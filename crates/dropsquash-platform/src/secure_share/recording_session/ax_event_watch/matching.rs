use std::ptr::NonNull;

use dropsquash_core::CaptureRect;
use objc2_application_services::AXUIElement;
use objc2_core_foundation::{CFArray, CFRetained, CGPoint, CGSize};

use super::super::super::ax_observation::{copy_attribute, copy_ax_point, copy_ax_size};

pub(super) fn window(
    pid: i32,
    frame: CaptureRect,
) -> std::result::Result<CFRetained<AXUIElement>, String> {
    let app = unsafe { AXUIElement::new_application(pid) };
    let windows = copy_attribute(&app, "AXWindows").map_err(|error| error.to_string())?;
    let windows = windows
        .downcast::<CFArray>()
        .map_err(|_| "AXWindows was not an array")?;
    (0..windows.count())
        .filter_map(|index| unsafe { windows.value_at_index(index).cast::<AXUIElement>().as_ref() })
        .find(|window| matches_frame(window, frame))
        .map(|window| unsafe {
            let raw: NonNull<AXUIElement> = NonNull::from(window);
            CFRetained::retain(raw)
        })
        .ok_or_else(|| "Accessibility did not match the selected window".to_string())
}

fn matches_frame(window: &AXUIElement, expected: CaptureRect) -> bool {
    let Ok(CGPoint { x, y }) = copy_ax_point(window, "AXPosition") else {
        return false;
    };
    let Ok(CGSize { width, height }) = copy_ax_size(window, "AXSize") else {
        return false;
    };
    (x as i32, y as i32, width as u32, height as u32)
        == (expected.x, expected.y, expected.width, expected.height)
}
