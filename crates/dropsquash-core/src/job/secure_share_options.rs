use serde::{Deserialize, Serialize};

use crate::MaskPlan;

use super::{MaskMode, MaskRect};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecureShareOptions {
    pub mask_mode: MaskMode,
    pub mask_rects: Vec<MaskRect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mask_plan: Option<MaskPlan>,
}
