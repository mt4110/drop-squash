use serde::{Deserialize, Serialize};

use super::{FrameMaskPlan, MaskPlanAudit, VerificationExpectations};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrameSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaskPolicy {
    SmartMask,
    StrictReveal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaskPlan {
    pub schema_version: u32,
    pub capture_id: String,
    pub frame_size: FrameSize,
    pub frames: Vec<FrameMaskPlan>,
    pub policy: MaskPolicy,
    pub audit: MaskPlanAudit,
    pub verification_expectations: VerificationExpectations,
}
