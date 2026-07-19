use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaskReason {
    AxTextElement,
    AxModalBody,
    AxUnknownClientArea,
    VisionText,
    VisionTextShape,
    TransientOverlay,
    DynamicTitle,
    UrlOrFilename,
    Notification,
    UnknownRegion,
    VerificationRequired,
}
