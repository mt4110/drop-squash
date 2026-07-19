use serde::{Deserialize, Serialize};

use super::{MaskMode, MaskRect};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecureShareOptions {
    pub mask_mode: MaskMode,
    pub mask_rects: Vec<MaskRect>,
}
