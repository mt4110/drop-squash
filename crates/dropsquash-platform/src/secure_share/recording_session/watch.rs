use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use dropsquash_core::{AppError, Result};

use super::super::core_graphics_window::{changed_since, WindowAttestation};
use super::super::SckWindowSelection;

const WATCH_INTERVAL: Duration = Duration::from_millis(250);
const SNAPSHOT_TIMEOUT: Duration = Duration::from_secs(1);

pub(super) struct WindowWatch {
    selection: SckWindowSelection,
    attestation: WindowAttestation,
    display_epoch: u64,
    stopping: Arc<AtomicBool>,
    failure: Arc<Mutex<Option<String>>>,
    thread: Option<JoinHandle<()>>,
}

impl WindowWatch {
    pub(super) fn start(
        selection: SckWindowSelection,
        display_epoch: u64,
        attestation: WindowAttestation,
    ) -> Self {
        let stopping = Arc::new(AtomicBool::new(false));
        let failure = Arc::new(Mutex::new(None));
        let thread = spawn_watch(
            Arc::clone(&stopping),
            Arc::clone(&failure),
            selection.clone(),
            display_epoch,
            attestation,
        );
        Self {
            selection,
            attestation,
            display_epoch,
            stopping,
            failure,
            thread: Some(thread),
        }
    }

    pub(super) fn finish(mut self) -> Result<()> {
        self.stopping.store(true, Ordering::Release);
        let _ = self.thread.take().expect("watch thread exists").join();
        let failure = self.failure.lock().ok().and_then(|failure| failure.clone());
        let boundary_failure = changed_since(&self.selection, self.attestation).err();
        finish_result(
            super::super::display_watch::changed_since(self.display_epoch),
            failure.or(boundary_failure),
        )
    }
}

fn spawn_watch(
    stopping: Arc<AtomicBool>,
    failure: Arc<Mutex<Option<String>>>,
    selection: SckWindowSelection,
    display_epoch: u64,
    attestation: WindowAttestation,
) -> JoinHandle<()> {
    thread::spawn(move || {
        while !stopping.load(Ordering::Acquire) {
            if super::super::display_watch::changed_since(display_epoch) {
                store_failure(&failure, "macOS display configuration changed");
                return;
            }
            if let Err(reason) = changed_since(&selection, attestation) {
                store_failure(&failure, &reason);
                return;
            }
            if let Err(error) = super::revalidation::after_stop(&selection, SNAPSHOT_TIMEOUT) {
                store_failure(&failure, &error.to_string());
                return;
            }
            thread::sleep(WATCH_INTERVAL);
        }
    })
}

fn store_failure(failure: &Mutex<Option<String>>, reason: &str) {
    if let Ok(mut stored) = failure.lock() {
        *stored = Some(reason.to_string());
    }
}

fn finish_result(display_changed: bool, failure: Option<String>) -> Result<()> {
    if display_changed {
        return Err(fail_closed(
            "macOS display configuration changed".to_string(),
        ));
    }
    failure.map_or(Ok(()), |reason| Err(fail_closed(reason)))
}

fn fail_closed(reason: String) -> AppError {
    AppError::InvalidConfig(format!(
        "Secure Share recording target changed or could not be revalidated during capture: {reason}"
    ))
}

#[cfg(test)]
mod tests {
    use super::finish_result;

    #[test]
    fn display_change_rejects_even_without_a_target_failure() {
        let error = finish_result(true, None).expect_err("display change must fail closed");
        assert!(error.to_string().contains("display configuration changed"));
    }

    #[test]
    fn display_change_takes_priority_over_a_target_failure() {
        let error = finish_result(true, Some("target changed".into()))
            .expect_err("display change must fail closed");
        assert!(error.to_string().contains("display configuration changed"));
    }
}
