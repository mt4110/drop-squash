use std::ffi::c_void;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc::SyncSender, Arc};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use dropsquash_core::CaptureRect;
use objc2_application_services::{AXError, AXObserver, AXUIElement};
use objc2_core_foundation::{kCFRunLoopDefaultMode, CFRetained, CFRunLoop, CFString};

use super::matching;

const POLL: Duration = Duration::from_millis(20);

type RegisteredWatch = (
    CFRetained<AXObserver>,
    CFRetained<AXUIElement>,
    CFRetained<CFRunLoop>,
    CFRetained<objc2_core_foundation::CFRunLoopSource>,
    *mut c_void,
);

pub(super) fn spawn(
    pid: i32,
    frame: CaptureRect,
    stopping: Arc<AtomicBool>,
    changed: Arc<AtomicBool>,
    ready: SyncSender<std::result::Result<(), String>>,
) -> JoinHandle<()> {
    thread::spawn(move || run(pid, frame, stopping, changed, ready))
}

fn run(
    pid: i32,
    frame: CaptureRect,
    stopping: Arc<AtomicBool>,
    changed: Arc<AtomicBool>,
    ready: SyncSender<std::result::Result<(), String>>,
) {
    let result = setup(pid, frame, Arc::clone(&changed));
    let Ok((observer, window, run_loop, source, refcon)) = result else {
        let _ = ready.send(Err(result.expect_err("result was checked")));
        return;
    };
    let _ = ready.send(Ok(()));
    let mode = unsafe { kCFRunLoopDefaultMode };
    while !stopping.load(Ordering::Acquire) && !changed.load(Ordering::Acquire) {
        let _ = CFRunLoop::run_in_mode(mode, POLL.as_secs_f64(), true);
    }
    run_loop.remove_source(Some(&source), mode);
    drop(observer);
    drop(window);
    unsafe { drop(Box::from_raw(refcon.cast::<Arc<AtomicBool>>())) };
}

fn setup(
    pid: i32,
    frame: CaptureRect,
    changed: Arc<AtomicBool>,
) -> std::result::Result<RegisteredWatch, String> {
    let window = matching::window(pid, frame)?;
    let refcon = Box::into_raw(Box::new(changed)).cast::<c_void>();
    let result = (|| {
        let observer = create_observer(pid)?;
        for notification in ["AXMoved", "AXResized"] {
            add_notification(&observer, &window, notification, refcon)?;
        }
        let run_loop = CFRunLoop::current().ok_or("Accessibility event run loop unavailable")?;
        let source = unsafe { observer.run_loop_source() };
        let mode = unsafe { kCFRunLoopDefaultMode };
        run_loop.add_source(Some(&source), mode);
        Ok((observer, window, run_loop, source, refcon))
    })();
    if result.is_err() {
        unsafe { drop(Box::from_raw(refcon.cast::<Arc<AtomicBool>>())) };
    }
    result
}

fn create_observer(pid: i32) -> std::result::Result<CFRetained<AXObserver>, String> {
    let mut raw = std::ptr::null_mut();
    let output = NonNull::from(&mut raw);
    let error = unsafe { AXObserver::create(pid, Some(callback), output) };
    if error != AXError::Success {
        return Err(format!("Accessibility observer creation failed: {error:?}"));
    }
    let raw = NonNull::new(raw).ok_or("Accessibility observer was null")?;
    Ok(unsafe { CFRetained::from_raw(raw) })
}

fn add_notification(
    observer: &AXObserver,
    window: &AXUIElement,
    notification: &str,
    refcon: *mut c_void,
) -> std::result::Result<(), String> {
    let name = CFString::from_str(notification);
    let error = unsafe { observer.add_notification(window, &name, refcon) };
    (error == AXError::Success)
        .then_some(())
        .ok_or_else(|| format!("Accessibility {notification} registration failed: {error:?}"))
}

unsafe extern "C-unwind" fn callback(
    _: NonNull<AXObserver>,
    _: NonNull<AXUIElement>,
    _: NonNull<CFString>,
    refcon: *mut c_void,
) {
    if let Some(changed) = unsafe { refcon.cast::<Arc<AtomicBool>>().as_ref() } {
        changed.store(true, Ordering::Release);
    }
}
