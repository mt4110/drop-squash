use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread::JoinHandle;
use std::time::Duration;

use dropsquash_core::{AppError, Result};

use super::super::SckWindowSelection;

mod matching;
mod setup;

const START_TIMEOUT: Duration = Duration::from_secs(1);

pub(super) struct AxEventWatch {
    stopping: Arc<AtomicBool>,
    changed: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl AxEventWatch {
    pub(super) fn start(selection: &SckWindowSelection) -> Result<Self> {
        let stopping = Arc::new(AtomicBool::new(false));
        let changed = Arc::new(AtomicBool::new(false));
        let (ready, receiver) = mpsc::sync_channel(1);
        let thread = setup::spawn(
            selection.owner_pid,
            selection.frame,
            Arc::clone(&stopping),
            Arc::clone(&changed),
            ready,
        );
        match receiver.recv_timeout(START_TIMEOUT) {
            Ok(Ok(())) => Ok(Self {
                stopping,
                changed,
                thread: Some(thread),
            }),
            Ok(Err(reason)) => Err(fail_closed(reason)),
            Err(_) => {
                stopping.store(true, Ordering::Release);
                let _ = thread.join();
                Err(fail_closed("Accessibility event watch did not start"))
            }
        }
    }

    pub(super) fn finish(mut self) -> Result<()> {
        self.stopping.store(true, Ordering::Release);
        let _ = self
            .thread
            .take()
            .expect("AX event watch thread exists")
            .join();
        if self.changed.load(Ordering::Acquire) {
            return Err(fail_closed(
                "Accessibility reported selected window geometry changed",
            ));
        }
        Ok(())
    }
}

fn fail_closed(reason: impl AsRef<str>) -> AppError {
    AppError::InvalidConfig(format!(
        "Secure Share Accessibility event watch failed closed: {}",
        reason.as_ref()
    ))
}

#[cfg(test)]
mod tests {
    use super::fail_closed;

    #[test]
    fn startup_failure_names_the_required_watcher() {
        assert!(fail_closed("missing").to_string().contains("event watch"));
    }
}
