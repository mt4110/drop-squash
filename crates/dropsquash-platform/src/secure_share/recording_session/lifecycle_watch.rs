use block2::RcBlock;
use dropsquash_core::{AppError, Result};
use objc2::rc::Retained;
use objc2::runtime::{NSObjectProtocol, ProtocolObject};
use objc2_app_kit::{
    NSWorkspace, NSWorkspaceActiveSpaceDidChangeNotification,
    NSWorkspaceScreensDidSleepNotification, NSWorkspaceSessionDidResignActiveNotification,
    NSWorkspaceWillPowerOffNotification, NSWorkspaceWillSleepNotification,
};
use objc2_foundation::{NSNotification, NSNotificationCenter, NSNotificationName};
use std::ptr::NonNull;
use std::sync::{Arc, Mutex};

pub(super) struct LifecycleWatch {
    center: Retained<NSNotificationCenter>,
    observers: Vec<Retained<ProtocolObject<dyn NSObjectProtocol>>>,
    reason: Arc<Mutex<Option<&'static str>>>,
}

impl LifecycleWatch {
    pub(super) fn start() -> Self {
        let center = NSWorkspace::sharedWorkspace().notificationCenter();
        let reason = Arc::new(Mutex::new(None));
        let observers = watched_notifications()
            .iter()
            .map(|(name, label)| observe(&center, name, label, Arc::clone(&reason)))
            .collect();
        Self {
            center,
            observers,
            reason,
        }
    }

    pub(super) fn finish(self) -> Result<()> {
        for observer in &self.observers {
            let observer: &ProtocolObject<dyn NSObjectProtocol> = observer;
            unsafe { self.center.removeObserver(observer.as_ref()) };
        }
        result_from(self.reason.lock().ok().and_then(|reason| *reason))
    }
}

fn observe(
    center: &NSNotificationCenter,
    name: &NSNotificationName,
    label: &'static str,
    reason: Arc<Mutex<Option<&'static str>>>,
) -> Retained<ProtocolObject<dyn NSObjectProtocol>> {
    let block = RcBlock::new(move |_note: NonNull<NSNotification>| {
        if let Ok(mut stored) = reason.lock() {
            *stored = Some(label);
        }
    });
    unsafe { center.addObserverForName_object_queue_usingBlock(Some(name), None, None, &block) }
}

fn watched_notifications() -> [(&'static NSNotificationName, &'static str); 5] {
    [
        (
            unsafe { NSWorkspaceSessionDidResignActiveNotification },
            "macOS user session resigned active",
        ),
        (
            unsafe { NSWorkspaceWillSleepNotification },
            "macOS workspace will sleep",
        ),
        (
            unsafe { NSWorkspaceScreensDidSleepNotification },
            "macOS screens slept",
        ),
        (
            unsafe { NSWorkspaceWillPowerOffNotification },
            "macOS workspace will power off",
        ),
        (
            unsafe { NSWorkspaceActiveSpaceDidChangeNotification },
            "macOS active Space changed",
        ),
    ]
}

fn fail_closed(reason: &str) -> AppError {
    AppError::InvalidConfig(format!(
        "Secure Share macOS lifecycle changed during capture: {reason}"
    ))
}

fn result_from(reason: Option<&'static str>) -> Result<()> {
    reason.map_or(Ok(()), |reason| Err(fail_closed(reason)))
}

#[cfg(test)]
mod tests;
