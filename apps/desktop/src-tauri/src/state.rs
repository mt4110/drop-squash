use std::sync::Mutex;

use dropsquash_core::{EncodeJob, EncodeResult};
use dropsquash_queue::{QueueEvent, QueueItem, QueueJobId, QueueWorker};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Default)]
pub struct AppState {
    pub config_lock: Mutex<()>,
    pub active_conversion: Mutex<Option<CancellationToken>>,
    queue: Mutex<QueueWorker>,
}

impl AppState {
    pub fn start_conversion(&self) -> Result<CancellationToken, String> {
        let mut active = self.active_conversion.lock().map_err(lock_error)?;
        if active.is_some() {
            return Err("A conversion is already running.".to_string());
        }
        let token = CancellationToken::new();
        *active = Some(token.clone());
        Ok(token)
    }

    pub fn cancel_conversion(&self) -> Result<bool, String> {
        let active = self.active_conversion.lock().map_err(lock_error)?;
        Ok(active.as_ref().is_some_and(|token| {
            token.cancel();
            true
        }))
    }

    pub fn finish_conversion(&self) {
        if let Ok(mut active) = self.active_conversion.lock() {
            *active = None;
        }
    }

    pub fn enqueue_job(&self, job: EncodeJob) -> Result<QueueEvent, String> {
        Ok(self.queue.lock().map_err(lock_error)?.enqueue(job))
    }

    pub fn start_next_job(&self) -> Result<Option<QueueEvent>, String> {
        Ok(self.queue.lock().map_err(lock_error)?.start_next())
    }

    pub fn finish_active_queue_job(
        &self,
        result: EncodeResult,
    ) -> Result<Option<QueueEvent>, String> {
        Ok(self.queue.lock().map_err(lock_error)?.finish_active(result))
    }

    pub fn fail_active_queue_job(&self, error: String) -> Result<Option<QueueEvent>, String> {
        Ok(self.queue.lock().map_err(lock_error)?.fail_active(error))
    }

    pub fn cancel_active_queue_job(&self) -> Result<Option<QueueEvent>, String> {
        Ok(self.queue.lock().map_err(lock_error)?.cancel_active())
    }

    pub fn cancel_queued_job(&self, id: QueueJobId) -> Result<Option<QueueEvent>, String> {
        Ok(self.queue.lock().map_err(lock_error)?.cancel_pending(id))
    }

    pub fn block_queued_jobs(&self, error: String) -> Result<Vec<QueueEvent>, String> {
        Ok(self.queue.lock().map_err(lock_error)?.block_pending(error))
    }

    pub fn clear_completed_jobs(&self) -> Result<Vec<QueueItem>, String> {
        Ok(self.queue.lock().map_err(lock_error)?.clear_completed())
    }
}

fn lock_error<T>(_error: T) -> String {
    "conversion state lock was poisoned".to_string()
}

#[cfg(test)]
mod tests;
