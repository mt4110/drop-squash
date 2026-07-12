use std::collections::VecDeque;

use dropsquash_core::EncodeJob;

use crate::{QueueItem, QueueJobId, QueueJobStatus};

#[derive(Debug, Default)]
pub struct InMemoryQueue {
    next_id: u64,
    pending: VecDeque<QueueItem>,
    active: Option<QueueItem>,
    completed: Vec<QueueItem>,
}

impl InMemoryQueue {
    pub fn enqueue(&mut self, job: EncodeJob) -> QueueItem {
        self.next_id += 1;
        let item = QueueItem::queued(QueueJobId(self.next_id), job);
        self.pending.push_back(item.clone());
        item
    }

    pub fn start_next(&mut self) -> Option<QueueItem> {
        if self.active.is_some() {
            return None;
        }
        let item = self.pending.pop_front()?.running();
        self.active = Some(item.clone());
        Some(item)
    }

    pub fn finish_active(&mut self) -> Option<QueueItem> {
        self.complete_active(QueueJobStatus::Succeeded, None)
    }

    pub fn fail_active(&mut self, error: String) -> Option<QueueItem> {
        self.complete_active(QueueJobStatus::Failed, Some(error))
    }

    pub fn cancel_active(&mut self) -> Option<QueueItem> {
        self.complete_active(QueueJobStatus::Cancelled, None)
    }

    pub fn cancel_pending(&mut self, id: QueueJobId) -> Option<QueueItem> {
        let index = self.pending.iter().position(|item| item.id == id)?;
        let item = self.pending.remove(index)?.cancelled();
        self.completed.push(item.clone());
        Some(item)
    }

    pub fn block_pending(&mut self, error: String) -> Vec<QueueItem> {
        let blocked = self
            .pending
            .drain(..)
            .map(|item| item.blocked(error.clone()))
            .collect::<Vec<_>>();
        self.completed.extend(blocked.iter().cloned());
        blocked
    }

    pub fn clear_completed(&mut self) -> Vec<QueueItem> {
        self.completed.drain(..).collect()
    }

    pub fn active(&self) -> Option<&QueueItem> {
        self.active.as_ref()
    }

    pub fn completed(&self) -> &[QueueItem] {
        &self.completed
    }

    pub fn len(&self) -> usize {
        self.pending.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pending.is_empty() && self.active.is_none()
    }

    fn complete_active(
        &mut self,
        status: QueueJobStatus,
        error: Option<String>,
    ) -> Option<QueueItem> {
        let mut item = self.active.take()?;
        item.status = status;
        item.error = error;
        self.completed.push(item.clone());
        Some(item)
    }
}

#[cfg(test)]
mod tests;
