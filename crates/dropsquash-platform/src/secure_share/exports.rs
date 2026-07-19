pub use super::attachments::{
    core_media_attachment_binding_name, ensure_required_sck_frame_attachments,
    raw_sck_frame_info_from_sample_buffer, NativeSampleBuffer, SckFrameAttachmentPresence,
};
pub use super::frame_info::{RawSckFrameInfo, RawSckFrameStatus};
pub use super::native_frame::{
    screen_capture_kit_binding_name, NativeFrameInfoKey, NativeFrameMetadataProvider,
};
#[cfg(target_os = "macos")]
pub use super::native_vision::{
    sample_buffer_request_handler, vision_binding_name, NativeVisionImageOption,
    NativeVisionImageRequestHandler, NativeVisionObservationProvider, NativeVisionRequest,
    NativeVisionTextRequest, NativeVisionTextShapeRequest,
};
#[cfg(target_os = "macos")]
pub use super::native_vision_request::{
    make_text_request, make_text_shape_request, perform_text_requests, NativeVisionRequests,
};
#[cfg(target_os = "macos")]
pub use super::native_vision_result::{text_observations, text_shape_observations};
#[cfg(target_os = "macos")]
pub use super::observation_callback::observe_window_once_callback;
#[cfg(target_os = "macos")]
pub use super::observation_probe::{
    observe_strict_reveal_window_once, observe_window_once, SckObservationProbeReport,
    SckObservationProbeRequest,
};
pub use super::providers::{
    capture_snapshot_from_providers, AccessibilityObservationProvider,
    EmptyAccessibilityObservationProvider, EmptyVisionObservationProvider, FrameMetadataProvider,
    VisionObservationProvider,
};

#[cfg(target_os = "macos")]
pub use super::live_mask::SckLiveMaskEvidence;
#[cfg(target_os = "macos")]
pub use super::sample_buffer_provider::{
    NativeSampleBufferProvider, SampleBufferFrameMetadataProvider,
};
#[cfg(target_os = "macos")]
pub use super::shareable_content::{
    snapshot_shareable_content, SckDisplayCandidate, SckShareableContentSnapshot,
    SckWindowCandidate,
};
#[cfg(target_os = "macos")]
pub use super::shareable_request::{
    request_shareable_content_snapshot, SckShareableContentRequest,
};
#[cfg(target_os = "macos")]
pub use super::stream_output::SckStreamFrameMetadataOutput;
#[cfg(target_os = "macos")]
pub use super::stream_plan::{build_stream_capture_plan, SckStreamCapturePlan};
#[cfg(target_os = "macos")]
pub use super::stream_registration::SckFrameMetadataStreamRegistration;
#[cfg(target_os = "macos")]
pub use super::target_policy::{
    select_explicit_display_target, select_explicit_window_target,
    select_strict_reveal_window_target, SckCaptureTarget, SckCaptureTargetKind,
};
