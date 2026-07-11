use dropsquash_core::{EncodeJob, EncodeResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueueEvent {
    Enqueued(EncodeJob),
    Started(EncodeJob),
    Finished(EncodeResult),
    Failed(String),
}
