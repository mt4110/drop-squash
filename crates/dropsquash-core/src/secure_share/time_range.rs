use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeRangeNs {
    pub start_ns: u64,
    pub end_ns: u64,
}

impl TimeRangeNs {
    pub fn contains(self, time_ns: u64) -> bool {
        self.start_ns <= time_ns && time_ns < self.end_ns
    }
}
