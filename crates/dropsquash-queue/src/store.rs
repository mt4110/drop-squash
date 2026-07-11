use std::collections::VecDeque;

use dropsquash_core::EncodeJob;

#[derive(Debug, Default)]
pub struct InMemoryQueue {
    jobs: VecDeque<EncodeJob>,
}

impl InMemoryQueue {
    pub fn push(&mut self, job: EncodeJob) {
        self.jobs.push_back(job);
    }

    pub fn pop(&mut self) -> Option<EncodeJob> {
        self.jobs.pop_front()
    }

    pub fn len(&self) -> usize {
        self.jobs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.jobs.is_empty()
    }
}
