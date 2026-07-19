use serde::{Deserialize, Serialize};

use super::{MaskReason, ObservationSource, TimeRangeNs};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaskPlanAudit {
    pub unmatched_observations: Vec<UnmatchedObservation>,
    pub verification_required_frame_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnmatchedObservation {
    pub source: ObservationSource,
    pub reason: MaskReason,
    pub time_range: TimeRangeNs,
}

impl MaskPlanAudit {
    pub fn clean() -> Self {
        Self {
            unmatched_observations: Vec::new(),
            verification_required_frame_count: 0,
        }
    }

    pub fn record_unmatched(
        &mut self,
        source: ObservationSource,
        reason: MaskReason,
        time_range: TimeRangeNs,
    ) {
        self.unmatched_observations.push(UnmatchedObservation {
            source,
            reason,
            time_range,
        });
    }
}
