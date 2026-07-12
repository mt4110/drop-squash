use dropsquash_core::{EncodeJob, EncodeResult};

use crate::{InMemoryQueue, QueueEvent, QueueItem, QueueJobId};

#[derive(Debug, Default)]
pub struct QueueWorker {
    queue: InMemoryQueue,
}

impl QueueWorker {
    pub fn enqueue(&mut self, job: EncodeJob) -> QueueEvent {
        QueueEvent::Enqueued(self.queue.enqueue(job))
    }

    pub fn start_next(&mut self) -> Option<QueueEvent> {
        self.queue.start_next().map(QueueEvent::Started)
    }

    pub fn finish_active(&mut self, result: EncodeResult) -> Option<QueueEvent> {
        let item = self.queue.finish_active()?;
        Some(QueueEvent::Finished {
            id: item.id,
            result,
        })
    }

    pub fn fail_active(&mut self, error: String) -> Option<QueueEvent> {
        let item = self.queue.fail_active(error.clone())?;
        Some(QueueEvent::Failed { id: item.id, error })
    }

    pub fn cancel_active(&mut self) -> Option<QueueEvent> {
        self.queue
            .cancel_active()
            .map(|item| QueueEvent::Cancelled(item.id))
    }

    pub fn cancel_pending(&mut self, id: QueueJobId) -> Option<QueueEvent> {
        self.queue
            .cancel_pending(id)
            .map(|item| QueueEvent::Cancelled(item.id))
    }

    pub fn block_pending(&mut self, error: String) -> Vec<QueueEvent> {
        self.queue
            .block_pending(error.clone())
            .into_iter()
            .map(|item| QueueEvent::Blocked {
                id: item.id,
                error: error.clone(),
            })
            .collect()
    }

    pub fn clear_completed(&mut self) -> Vec<QueueItem> {
        self.queue.clear_completed()
    }

    pub fn active(&self) -> Option<&QueueItem> {
        self.queue.active()
    }
}

#[cfg(test)]
mod tests;
