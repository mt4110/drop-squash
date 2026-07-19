use serde::{Deserialize, Serialize};

use super::{
    Confidence, MaskReason, MaskRegion, ObservationSource, PixelRect, RegionPolicy, TimeRangeNs,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AxObservationKind {
    TextElement,
    ModalBody,
    UnknownClientArea,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AxObservation {
    pub rect: PixelRect,
    pub time_range: TimeRangeNs,
    pub kind: AxObservationKind,
    pub confidence: Confidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisionObservationKind {
    TextRecognition,
    TextRectangle,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisionObservation {
    pub rect: PixelRect,
    pub time_range: TimeRangeNs,
    pub kind: VisionObservationKind,
    pub confidence: Confidence,
}

impl AxObservation {
    pub fn to_region(self) -> MaskRegion {
        let (reason, source) = match self.kind {
            AxObservationKind::TextElement => (
                MaskReason::AxTextElement,
                ObservationSource::AccessibilityText,
            ),
            AxObservationKind::ModalBody => (
                MaskReason::AxModalBody,
                ObservationSource::AccessibilityModal,
            ),
            AxObservationKind::UnknownClientArea => (
                MaskReason::AxUnknownClientArea,
                ObservationSource::AccessibilityWindow,
            ),
        };
        region(self.rect, reason, source, self.confidence)
    }
}

impl VisionObservation {
    pub fn to_region(self) -> MaskRegion {
        let (reason, source) = match self.kind {
            VisionObservationKind::TextRecognition => (
                MaskReason::VisionText,
                ObservationSource::VisionTextRecognition,
            ),
            VisionObservationKind::TextRectangle => (
                MaskReason::VisionTextShape,
                ObservationSource::VisionTextRectangle,
            ),
        };
        region(self.rect, reason, source, self.confidence)
    }
}

fn region(
    rect: PixelRect,
    reason: MaskReason,
    source: ObservationSource,
    confidence: Confidence,
) -> MaskRegion {
    MaskRegion {
        rect,
        policy: RegionPolicy::Sensitive,
        reason,
        sources: vec![source],
        confidence,
        expansion_px: 8,
    }
}
