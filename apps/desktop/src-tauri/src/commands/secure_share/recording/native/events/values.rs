use dropsquash_core::CaptureFrameMetadata;
use dropsquash_platform::{
    NativeAccessibilityObservation, NativeDestructionEvidence, NativeTemporalObservation,
    NativeVisionObservation,
};

pub(super) type StoppedValues = (
    Vec<CaptureFrameMetadata>,
    Vec<NativeVisionObservation>,
    Vec<NativeAccessibilityObservation>,
    Vec<NativeTemporalObservation>,
    Vec<NativeDestructionEvidence>,
);
