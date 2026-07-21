use std::ffi::c_void;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::OnceLock;

use dropsquash_core::{AppError, Result};

static DISPLAY_EPOCH: AtomicU64 = AtomicU64::new(0);
static REGISTERED: OnceLock<bool> = OnceLock::new();

pub(super) fn start() -> Result<u64> {
    if !*REGISTERED.get_or_init(register) {
        return Err(AppError::InvalidConfig(
            "Secure Share could not watch macOS display configuration".to_string(),
        ));
    }
    Ok(DISPLAY_EPOCH.load(Ordering::Acquire))
}

pub(super) fn changed_since(epoch: u64) -> bool {
    changed(epoch, DISPLAY_EPOCH.load(Ordering::Acquire))
}

fn register() -> bool {
    unsafe {
        CGDisplayRegisterReconfigurationCallback(Some(display_changed), std::ptr::null_mut()) == 0
    }
}

unsafe extern "C" fn display_changed(_display: u32, _flags: u32, _user_info: *mut c_void) {
    DISPLAY_EPOCH.fetch_add(1, Ordering::Release);
}

#[link(name = "CoreGraphics", kind = "framework")]
unsafe extern "C" {
    fn CGDisplayRegisterReconfigurationCallback(
        callback: Option<unsafe extern "C" fn(u32, u32, *mut c_void)>,
        user_info: *mut c_void,
    ) -> i32;
}

fn changed(epoch: u64, current: u64) -> bool {
    current != epoch
}

#[cfg(test)]
mod tests {
    use super::changed;

    #[test]
    fn detects_a_display_epoch_change() {
        assert!(!changed(7, 7));
        assert!(changed(7, 8));
    }
}
