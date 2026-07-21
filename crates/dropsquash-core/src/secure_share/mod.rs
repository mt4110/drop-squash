mod agreement;
mod audit;
mod builder;
mod capture;
mod coalesce;
mod confidence;
mod continuity_watch;
mod coverage;
mod export_options;
mod exposure_coverage;
mod frame;
mod observation;
mod plan;
mod policy;
mod precision;
mod reason;
mod rect;
mod resolver;
mod smart_mask;
mod source;
mod time_range;
mod transform;
mod verification;
mod vision_transform;

pub use audit::{MaskPlanAudit, UnmatchedObservation};
pub use builder::MaskPlanDraft;
pub use capture::{CaptureFrameMetadata, CaptureRect};
pub use confidence::Confidence;
pub use continuity_watch::required_capture_continuity_watches;
pub use coverage::{destructive_coverage, DestructiveCoverage};
pub use export_options::mask_options_for_frame;
pub use exposure_coverage::{
    required_exposure_paths, strict_shield_exposure_coverage, ExposureCoverage,
    ExposureCoverageStatus,
};
pub use frame::{FrameMaskPlan, FrameStatus};
pub use observation::{
    AxObservation, AxObservationKind, TemporalObservation, VisionObservation, VisionObservationKind,
};
pub use plan::{FrameSize, MaskPlan, MaskPolicy};
pub use policy::RegionPolicy;
pub use precision::{
    selective_mask_precision, selective_mask_precision_for_frames, SelectiveMaskPrecision,
};
pub use reason::MaskReason;
pub use rect::{MaskRegion, PixelRect};
pub use source::ObservationSource;
pub use time_range::TimeRangeNs;
pub use transform::CoordinateSpace;
pub use verification::VerificationExpectations;
pub use vision_transform::VisionNormalizedRect;

#[cfg(test)]
mod tests;
