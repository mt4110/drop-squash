use dropsquash_core::EncodeJob;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct QueueJobId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueueJobStatus {
    Queued,
    Running,
    Succeeded,
    Failed,
    Cancelled,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueItem {
    pub id: QueueJobId,
    pub job: EncodeJob,
    pub status: QueueJobStatus,
    pub error: Option<String>,
}

impl QueueItem {
    pub fn queued(id: QueueJobId, job: EncodeJob) -> Self {
        Self {
            id,
            job,
            status: QueueJobStatus::Queued,
            error: None,
        }
    }

    pub fn running(mut self) -> Self {
        self.status = QueueJobStatus::Running;
        self
    }

    pub fn failed(mut self, error: String) -> Self {
        self.status = QueueJobStatus::Failed;
        self.error = Some(error);
        self
    }

    pub fn blocked(mut self, error: String) -> Self {
        self.status = QueueJobStatus::Blocked;
        self.error = Some(error);
        self
    }

    pub fn cancelled(mut self) -> Self {
        self.status = QueueJobStatus::Cancelled;
        self
    }
}
