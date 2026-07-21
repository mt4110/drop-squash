use crate::QueueEvent;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct QueueSummary {
    pub total: usize,
    pub finished: usize,
    pub succeeded: usize,
    pub unchanged: usize,
    pub failed: usize,
    pub cancelled: usize,
    pub blocked: usize,
    pub saved_bytes: u64,
}

impl QueueSummary {
    pub fn from_events(events: &[QueueEvent]) -> Self {
        let mut summary = Self::default();
        for event in events {
            summary.record(event);
        }
        summary
    }

    fn record(&mut self, event: &QueueEvent) {
        match event {
            QueueEvent::Enqueued(_) => self.total += 1,
            QueueEvent::Finished { result, .. } => {
                self.finished += 1;
                if result.is_successful_conversion() {
                    self.succeeded += 1;
                    self.saved_bytes += result.saved_bytes();
                } else {
                    self.failed += 1;
                }
            }
            QueueEvent::Failed { .. } => {
                self.finished += 1;
                self.failed += 1;
            }
            QueueEvent::Unchanged { .. } => {
                self.finished += 1;
                self.unchanged += 1;
            }
            QueueEvent::Cancelled(_) => {
                self.finished += 1;
                self.cancelled += 1;
            }
            QueueEvent::Blocked { .. } => {
                self.finished += 1;
                self.blocked += 1;
            }
            QueueEvent::Started(_) => {}
        }
    }
}

#[cfg(test)]
mod tests;
