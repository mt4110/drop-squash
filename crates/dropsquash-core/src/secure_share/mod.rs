mod audit;
mod builder;
mod capture;
mod coalesce;
mod confidence;
mod export_options;
mod frame;
mod observation;
mod plan;
mod policy;
mod reason;
mod rect;
mod resolver;
mod source;
mod time_range;
mod transform;
mod verification;
mod vision_transform;

pub use audit::{MaskPlanAudit, UnmatchedObservation};
pub use builder::MaskPlanDraft;
pub use capture::{CaptureFrameMetadata, CaptureRect};
pub use confidence::Confidence;
pub use export_options::mask_options_for_frame;
pub use frame::{FrameMaskPlan, FrameStatus};
pub use observation::{AxObservation, AxObservationKind, VisionObservation, VisionObservationKind};
pub use plan::{FrameSize, MaskPlan, MaskPolicy};
pub use policy::RegionPolicy;
pub use reason::MaskReason;
pub use rect::{MaskRegion, PixelRect};
pub use source::ObservationSource;
pub use time_range::TimeRangeNs;
pub use transform::CoordinateSpace;
pub use verification::VerificationExpectations;
pub use vision_transform::VisionNormalizedRect;

#[cfg(test)]
mod tests;
