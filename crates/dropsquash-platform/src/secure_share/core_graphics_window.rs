use std::ffi::c_void;

use dropsquash_core::{AppError, CaptureRect, Result};
use objc2_core_foundation::{
    CFArray, CFDictionary, CFNumber, CFNumberType, CFString, CFType, CGRect,
};
use objc2_core_graphics::{
    kCGNullWindowID, kCGWindowBounds, kCGWindowNumber, kCGWindowOwnerPID,
    CGRectMakeWithDictionaryRepresentation, CGWindowListCopyWindowInfo, CGWindowListOption,
};

use super::SckWindowSelection;

#[derive(Clone, Copy)]
pub(super) struct WindowAttestation {
    owner_pid: i32,
    frame: CaptureRect,
}

pub(super) fn attest(selection: &SckWindowSelection) -> Result<WindowAttestation> {
    let windows = CGWindowListCopyWindowInfo(
        CGWindowListOption::OptionOnScreenOnly | CGWindowListOption::ExcludeDesktopElements,
        kCGNullWindowID,
    )
    .ok_or_else(|| unavailable("window list unavailable"))?;
    let windows = unsafe { windows.cast_unchecked::<CFType>() };
    let found = (0..windows.len()).find_map(|index| window(windows, index, selection.window_id));
    let (owner_pid, frame) = found.ok_or_else(|| unavailable("selected window missing"))?;
    if owner_pid != selection.owner_pid {
        return Err(unavailable("selected window owner changed"));
    }
    Ok(WindowAttestation { owner_pid, frame })
}

pub(super) fn changed_since(
    selection: &SckWindowSelection,
    expected: WindowAttestation,
) -> std::result::Result<(), String> {
    let observed = attest(selection).map_err(|error| error.to_string())?;
    if !matches(expected, observed) {
        return Err("CoreGraphics selected window bounds changed".to_string());
    }
    Ok(())
}

fn matches(expected: WindowAttestation, observed: WindowAttestation) -> bool {
    expected.owner_pid == observed.owner_pid && expected.frame == observed.frame
}

fn window(windows: &CFArray<CFType>, index: usize, window_id: u32) -> Option<(i32, CaptureRect)> {
    let value = windows.get(index)?;
    let dictionary = value.downcast_ref::<CFDictionary>()?;
    let id = number(dictionary, unsafe { kCGWindowNumber })? as u32;
    if id != window_id {
        return None;
    }
    Some((
        number(dictionary, unsafe { kCGWindowOwnerPID })? as i32,
        rect(dictionary)?,
    ))
}

fn number(dictionary: &CFDictionary, key: &CFString) -> Option<i64> {
    let value = unsafe { dictionary.value(key as *const CFString as *const c_void) };
    let value = unsafe { (value as *const CFType).as_ref() }?;
    let value = value.downcast_ref::<CFNumber>()?;
    let mut number = 0_i64;
    unsafe {
        value.value(
            CFNumberType::SInt64Type,
            (&mut number as *mut i64).cast::<c_void>(),
        )
    }
    .then_some(number)
}

fn rect(dictionary: &CFDictionary) -> Option<CaptureRect> {
    let key = unsafe { kCGWindowBounds };
    let value = unsafe { dictionary.value(key as *const CFString as *const c_void) };
    let dictionary = unsafe { (value as *const CFDictionary).as_ref() }?;
    let mut frame = CGRect::default();
    unsafe { CGRectMakeWithDictionaryRepresentation(Some(dictionary), &mut frame) }.then_some(
        CaptureRect {
            x: frame.origin.x as i32,
            y: frame.origin.y as i32,
            width: frame.size.width as u32,
            height: frame.size.height as u32,
        },
    )
}

fn unavailable(reason: &str) -> AppError {
    AppError::InvalidConfig(format!(
        "Secure Share CoreGraphics target attestation failed closed: {reason}"
    ))
}

#[cfg(test)]
mod tests {
    use super::{matches, CaptureRect, WindowAttestation};

    #[test]
    fn changed_bounds_are_not_equivalent_to_the_baseline() {
        let baseline = WindowAttestation {
            owner_pid: 7,
            frame: CaptureRect {
                x: 0,
                y: 0,
                width: 800,
                height: 600,
            },
        };
        let resized = WindowAttestation {
            frame: CaptureRect {
                width: 801,
                ..baseline.frame
            },
            ..baseline
        };
        assert!(!matches(baseline, resized));
    }
}
