use serde::{Deserialize, Serialize};

use super::{ExposureCoverage, MaskReason, ObservationSource, TimeRangeNs};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MaskPlanAudit {
    pub unmatched_observations: Vec<UnmatchedObservation>,
    pub verification_required_frame_count: usize,
    pub accessibility_observation_count: usize,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub focused_text_observation_count: usize,
    pub vision_observation_count: usize,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub temporal_observation_count: usize,
    pub cross_source_overlap_count: usize,
    pub accessibility_only_count: usize,
    pub vision_only_count: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capture_continuity_attested: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capture_backend: Option<String>,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub native_destroyed_frame_count: usize,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub native_destroyed_region_count: usize,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub native_verified_frame_count: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capture_continuity_watches: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exposure_coverage: Vec<ExposureCoverage>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
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
            accessibility_observation_count: 0,
            focused_text_observation_count: 0,
            vision_observation_count: 0,
            temporal_observation_count: 0,
            cross_source_overlap_count: 0,
            accessibility_only_count: 0,
            vision_only_count: 0,
            capture_continuity_attested: None,
            capture_backend: None,
            native_destroyed_frame_count: 0,
            native_destroyed_region_count: 0,
            native_verified_frame_count: 0,
            capture_continuity_watches: Vec::new(),
            exposure_coverage: Vec::new(),
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

    pub(super) fn record_observation_agreement(
        &mut self,
        agreement: super::agreement::ObservationAgreement,
    ) {
        self.accessibility_observation_count = agreement.accessibility_count;
        self.vision_observation_count = agreement.vision_count;
        self.cross_source_overlap_count = agreement.overlapping_pair_count;
        self.accessibility_only_count = agreement.accessibility_only_count;
        self.vision_only_count = agreement.vision_only_count;
    }

    pub(super) fn record_temporal_count(&mut self, count: usize) {
        self.temporal_observation_count = count;
    }

    pub(super) fn record_focused_text_count(&mut self, count: usize) {
        self.focused_text_observation_count = count;
    }

    pub fn record_native_destruction(&mut self, frames: usize, regions: usize, verified: usize) {
        self.native_destroyed_frame_count = frames;
        self.native_destroyed_region_count = regions;
        self.native_verified_frame_count = verified;
    }
}

fn is_zero(value: &usize) -> bool {
    *value == 0
}
