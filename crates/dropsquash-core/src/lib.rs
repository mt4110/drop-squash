pub mod config;
pub mod error;
pub mod job;
pub mod paths;
pub mod result;
pub mod secure_share;

pub use config::{AppConfig, TRIAL_CONVERSION_LIMIT};
pub use error::{
    normalize_encoder_message, AppError, Result, CONVERSION_FAILED_MESSAGE,
    LICENSE_NETWORK_MESSAGE, NOT_SMALLER_MESSAGE,
};
pub use job::{
    EncodeJob, MaskMode, MaskRect, MediaInfo, OutputSize, Profile, SecureShareOptions, SourcePolicy,
};
pub use paths::{
    default_config_path, default_evidence_signing_key_path, default_history_path,
    default_license_cache_path, default_output_dir,
};
pub use result::{EncodeResult, LicenseState, LockedReason, TrialState};
pub use secure_share::{
    destructive_coverage, mask_options_for_frame, required_capture_continuity_watches,
    required_exposure_paths, selective_mask_precision, selective_mask_precision_for_frames,
    strict_shield_exposure_coverage, AxObservation, AxObservationKind, CaptureFrameMetadata,
    CaptureRect, Confidence, CoordinateSpace, DestructiveCoverage, ExposureCoverage,
    ExposureCoverageStatus, FrameMaskPlan, FrameSize, FrameStatus, MaskPlan, MaskPlanAudit,
    MaskPlanDraft, MaskPolicy, MaskReason, MaskRegion, ObservationSource, PixelRect, RegionPolicy,
    SelectiveMaskPrecision, TemporalObservation, TimeRangeNs, UnmatchedObservation,
    VerificationExpectations, VisionNormalizedRect, VisionObservation, VisionObservationKind,
};
