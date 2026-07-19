use std::ffi::c_void;
use std::ptr::NonNull;

use dropsquash_core::{AppError, AxObservation, Result, TimeRangeNs};
use objc2_application_services::{AXError, AXUIElement, AXValue, AXValueType};
use objc2_core_foundation::{
    CFArray, CFRetained, CFString, CFStringBuiltInEncodings, CFType, CGPoint, CGSize,
};

pub fn observe_ax_windows_for_pid(pid: i32, time_range: TimeRangeNs) -> Result<Vec<AxObservation>> {
    ensure_accessibility_access()?;
    let app = unsafe { AXUIElement::new_application(pid) };
    let windows = copy_attribute(&app, "AXWindows")?
        .downcast::<CFArray>()
        .map_err(|_| invalid("AXWindows was not an array"))?;
    observations_from_windows(&windows, time_range)
}

fn observations_from_windows(
    windows: &CFArray,
    time_range: TimeRangeNs,
) -> Result<Vec<AxObservation>> {
    let mut observations = Vec::new();
    for index in 0..windows.count() {
        let window = unsafe { windows.value_at_index(index) };
        if !window.is_null() {
            observations.push(observation_from_window(window, time_range)?);
        }
    }
    if observations.is_empty() {
        return Err(invalid("AXWindows returned no window rectangles"));
    }
    Ok(observations)
}

fn observation_from_window(
    window: *const c_void,
    time_range: TimeRangeNs,
) -> Result<AxObservation> {
    let window = unsafe { &*(window.cast::<AXUIElement>()) };
    super::ax_rect::observation_from_point_size(
        copy_ax_point(window, "AXPosition")?,
        copy_ax_size(window, "AXSize")?,
        time_range,
    )
}

fn copy_ax_point(element: &AXUIElement, name: &str) -> Result<CGPoint> {
    let value = copy_attribute(element, name)?
        .downcast::<AXValue>()
        .map_err(|_| invalid("AXPosition was not an AXValue"))?;
    decode_ax_value(&value, AXValueType::CGPoint)
}

fn copy_ax_size(element: &AXUIElement, name: &str) -> Result<CGSize> {
    let value = copy_attribute(element, name)?
        .downcast::<AXValue>()
        .map_err(|_| invalid("AXSize was not an AXValue"))?;
    decode_ax_value(&value, AXValueType::CGSize)
}

fn copy_attribute(element: &AXUIElement, name: &str) -> Result<CFRetained<CFType>> {
    let attribute = cf_string(name)?;
    let mut raw: *const CFType = std::ptr::null();
    let error = unsafe { element.copy_attribute_value(&attribute, NonNull::from(&mut raw)) };
    if error != AXError::Success {
        return Err(invalid(&format!("attribute {name} failed: {error:?}")));
    }
    let raw = NonNull::new(raw.cast_mut()).ok_or_else(|| invalid("AX attribute was null"))?;
    Ok(unsafe { CFRetained::from_raw(raw) })
}

fn decode_ax_value<T: Default>(value: &AXValue, value_type: AXValueType) -> Result<T> {
    let mut decoded = T::default();
    let ptr = NonNull::new((&mut decoded as *mut T).cast::<c_void>())
        .ok_or_else(|| invalid("AX decode pointer was null"))?;
    if unsafe { value.value(value_type, ptr) } {
        Ok(decoded)
    } else {
        Err(invalid("AXValue decode failed"))
    }
}

fn ensure_accessibility_access() -> Result<()> {
    if super::ax_permission::preflight_accessibility_access() {
        return Ok(());
    }
    Err(invalid("Accessibility permission is missing"))
}

fn cf_string(value: &str) -> Result<CFRetained<CFString>> {
    let c_string = std::ffi::CString::new(value).map_err(|_| invalid("AX attribute is invalid"))?;
    unsafe {
        CFString::with_c_string(
            None,
            c_string.as_ptr(),
            CFStringBuiltInEncodings::EncodingUTF8.0,
        )
    }
    .ok_or_else(|| invalid("AX attribute string allocation failed"))
}

fn invalid(reason: &str) -> AppError {
    AppError::InvalidConfig(format!("Secure Share Accessibility observation {reason}"))
}
