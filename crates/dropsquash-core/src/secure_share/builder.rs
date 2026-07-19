use serde::{Deserialize, Serialize};

use super::coalesce::coalesce_frame_regions;
use super::resolver::{assign_observation, count_verification_required_frames};
use super::{
    AxObservation, CaptureFrameMetadata, FrameSize, MaskPlan, MaskPlanAudit, MaskPolicy,
    VerificationExpectations, VisionObservation,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaskPlanDraft {
    pub capture_id: String,
    pub frame_size: FrameSize,
    pub frames: Vec<CaptureFrameMetadata>,
    pub accessibility: Vec<AxObservation>,
    pub vision: Vec<VisionObservation>,
    pub policy: MaskPolicy,
    pub verification_expectations: VerificationExpectations,
}

impl MaskPlanDraft {
    pub fn into_mask_plan(self) -> MaskPlan {
        let strict_reveal = self.policy == MaskPolicy::StrictReveal;
        let mut frames = self
            .frames
            .into_iter()
            .map(|frame| frame.to_mask_frame(self.frame_size, strict_reveal))
            .collect::<Vec<_>>();
        let mut audit = MaskPlanAudit::clean();
        for observation in self.accessibility {
            assign_observation(
                &mut frames,
                &mut audit,
                self.frame_size,
                strict_reveal,
                observation.time_range,
                observation.to_region(),
            );
        }
        for observation in self.vision {
            assign_observation(
                &mut frames,
                &mut audit,
                self.frame_size,
                strict_reveal,
                observation.time_range,
                observation.to_region(),
            );
        }
        coalesce_frame_regions(&mut frames);
        audit.verification_required_frame_count = count_verification_required_frames(&frames);
        MaskPlan {
            schema_version: 1,
            capture_id: self.capture_id,
            frame_size: self.frame_size,
            frames,
            policy: self.policy,
            audit,
            verification_expectations: self.verification_expectations,
        }
    }
}
