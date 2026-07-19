use serde::{Deserialize, Serialize};

use super::{Confidence, MaskReason, ObservationSource, RegionPolicy};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PixelRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaskRegion {
    pub rect: PixelRect,
    pub policy: RegionPolicy,
    pub reason: MaskReason,
    pub sources: Vec<ObservationSource>,
    pub confidence: Confidence,
    pub expansion_px: u32,
}
