use dropsquash_core::EncodeResult;
use serde::{Deserialize, Serialize};

use crate::{QueueItem, QueueJobId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueueEvent {
    Enqueued(QueueItem),
    Started(QueueItem),
    Finished {
        id: QueueJobId,
        result: EncodeResult,
    },
    Failed {
        id: QueueJobId,
        error: String,
    },
    Blocked {
        id: QueueJobId,
        error: String,
    },
    Cancelled(QueueJobId),
}
