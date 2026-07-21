use block2::RcBlock;
use dropsquash_core::{AppError, Result};
use objc2::rc::Retained;
use objc2::runtime::{NSObjectProtocol, ProtocolObject};
use objc2_app_kit::{
    NSRunningApplication, NSWorkspace, NSWorkspaceApplicationKey,
    NSWorkspaceDidActivateApplicationNotification,
};
use objc2_foundation::{NSNotification, NSNotificationCenter};
use std::ptr::NonNull;
use std::sync::{Arc, Mutex};

pub(super) struct ActivationWatch {
    center: Retained<NSNotificationCenter>,
    observer: Retained<ProtocolObject<dyn NSObjectProtocol>>,
    rejected_pid: Arc<Mutex<Option<i32>>>,
}

impl ActivationWatch {
    pub(super) fn start(target_pid: i32) -> Self {
        let allowed = AllowedPids::new(target_pid, std::process::id() as i32);
        let center = NSWorkspace::sharedWorkspace().notificationCenter();
        let rejected_pid = Arc::new(Mutex::new(None));
        let observer = observe(&center, allowed, Arc::clone(&rejected_pid));
        Self {
            center,
            observer,
            rejected_pid,
        }
    }

    pub(super) fn finish(self) -> Result<()> {
        let observer: &ProtocolObject<dyn NSObjectProtocol> = &self.observer;
        unsafe { self.center.removeObserver(observer.as_ref()) };
        result_from(self.rejected_pid.lock().ok().and_then(|pid| *pid))
    }
}

#[derive(Clone, Copy)]
struct AllowedPids {
    target_pid: i32,
    recorder_pid: i32,
}

impl AllowedPids {
    fn new(target_pid: i32, recorder_pid: i32) -> Self {
        Self {
            target_pid,
            recorder_pid,
        }
    }

    fn accepts(self, pid: i32) -> bool {
        pid == self.target_pid || pid == self.recorder_pid
    }
}

fn observe(
    center: &NSNotificationCenter,
    allowed: AllowedPids,
    rejected_pid: Arc<Mutex<Option<i32>>>,
) -> Retained<ProtocolObject<dyn NSObjectProtocol>> {
    let block = RcBlock::new(move |note: NonNull<NSNotification>| {
        if let Some(pid) = activated_pid(unsafe { note.as_ref() }) {
            store_if_disallowed(&rejected_pid, allowed, pid);
        }
    });
    unsafe {
        center.addObserverForName_object_queue_usingBlock(
            Some(NSWorkspaceDidActivateApplicationNotification),
            None,
            None,
            &block,
        )
    }
}

fn activated_pid(note: &NSNotification) -> Option<i32> {
    let app = note
        .userInfo()?
        .objectForKey(unsafe { NSWorkspaceApplicationKey })?
        .downcast::<NSRunningApplication>()
        .ok()?;
    Some(app.processIdentifier())
}

fn store_if_disallowed(
    rejected_pid: &Mutex<Option<i32>>,
    allowed: AllowedPids,
    activated_pid: i32,
) {
    if !allowed.accepts(activated_pid) {
        if let Ok(mut stored) = rejected_pid.lock() {
            if stored.is_none() {
                *stored = Some(activated_pid);
            }
        }
    }
}

fn result_from(rejected_pid: Option<i32>) -> Result<()> {
    rejected_pid.map_or(Ok(()), |pid| Err(fail_closed(pid)))
}

fn fail_closed(pid: i32) -> AppError {
    let diagnostic =
        if std::env::var("DROP_SQUASH_QA_ACTIVATION_DIAGNOSTIC").is_ok_and(|value| value == "1") {
            format!(" (pid {pid})")
        } else {
            String::new()
        };
    AppError::InvalidConfig(format!(
        "Secure Share foreground application changed during capture: third-party application became active{diagnostic}"
    ))
}

#[cfg(test)]
mod tests;
