use serde::{Deserialize, Serialize};

use super::MaskRegion;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrameStatus {
    Complete,
    Idle,
    Blank,
    Started,
    Suspended,
    Stopped,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrameMaskPlan {
    pub frame_index: u64,
    pub presentation_time_ns: u64,
    pub frame_status: FrameStatus,
    pub regions: Vec<MaskRegion>,
}
