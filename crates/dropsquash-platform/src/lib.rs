mod apple_capture_bridge;
mod dialog;
mod install;
mod keychain;
mod notify;
mod secure_share;
mod trash;
mod watch;

pub use apple_capture_bridge::apple_capture_bridge_abi_version;
#[cfg(target_os = "macos")]
pub use apple_capture_bridge::{
    capture_window_frame, discover_secure_share_content, release_copied_frame,
    start_attested_native_strict_recording, start_attested_strict_recording_with_observations,
    start_attested_strict_recording_with_temporal_observations, start_native_strict_recording,
    start_strict_recording, start_strict_recording_with_metadata, stop_strict_recording,
    AttestedObservationCallbacks, AttestedRecordingRequest, AttestedTemporalCallbacks,
    DiscoveryCallback, FrameCallback, FrameMetadataCallback, NativeAccessibilityObservation,
    NativeFrameMetadata, NativeRecordingEvent, NativeRecordingFailure, NativeStrictRecordingHandle,
    NativeTemporalObservation, NativeVisionObservation, RecordingCallback,
    TemporalObservationCallback, VisionObservationCallback,
};
pub use dialog::DialogService;
pub use install::{
    copy_app_bundle, copy_current_app_to_applications, current_install_location,
    eject_mounted_volume, install_location_for_executable, open_installed_application,
    reveal_finder_item, validate_installed_app_path, validate_mounted_volume_path,
    ApplicationsInstall, InstallLocation, InstallerCleanup,
};
pub use keychain::KeychainStore;
pub use notify::NotificationService;
#[cfg(target_os = "macos")]
pub use secure_share::accessibility_observations_from_native;
pub use secure_share::{
    build_stream_capture_plan, build_stream_capture_plan_for_output,
    capture_frame_metadata_from_native, capture_snapshot_from_providers,
    core_media_attachment_binding_name, decoded_text_residual_report,
    ensure_required_sck_frame_attachments, make_text_request, make_text_shape_request,
    observe_pixel_buffer, observe_strict_reveal_window_once, observe_window_once,
    observe_window_once_callback, perform_text_requests, pixel_buffer_request_handler,
    raw_sck_frame_info_from_sample_buffer, record_window_once, record_window_once_callback,
    record_window_once_callback_with_policy, record_window_until_stopped_callback,
    request_screen_capture_access, request_shareable_content_snapshot,
    sample_buffer_request_handler, screen_capture_access_granted, screen_capture_kit_binding_name,
    select_attested_window_target, select_explicit_display_target, select_explicit_window_target,
    select_strict_reveal_window_target, snapshot_shareable_content,
    temporal_observations_from_native, text_observations, text_shape_observations,
    vision_binding_name, vision_observations_from_native, AccessibilityObservationProvider,
    DecodedTextResidualReport, FrameMetadataProvider, NativeFrameInfoKey,
    NativeFrameMetadataProvider, NativeSampleBuffer, NativeSampleBufferProvider,
    NativeVisionImageOption, NativeVisionImageRequestHandler, NativeVisionObservationProvider,
    NativeVisionRequest, NativeVisionRequests, NativeVisionTextRequest,
    NativeVisionTextShapeRequest, RawSckFrameInfo, RawSckFrameStatus, RecordingCallbackRequest,
    SampleBufferFrameMetadataProvider, SckCaptureTarget, SckCaptureTargetKind, SckDisplayCandidate,
    SckFrameAttachmentPresence, SckFrameMetadataStreamRegistration, SckLiveMaskEvidence,
    SckObservationProbeReport, SckObservationProbeRequest, SckRecordingProbeReport,
    SckShareableContentRequest, SckShareableContentSnapshot, SckStreamCapturePlan,
    SckStreamFrameMetadataOutput, SckWindowCandidate, SckWindowSelection,
    SecureShareObservationSnapshot, SecureShareProbe, SecureShareSnapshotProvider,
    VisionObservationProvider,
};
pub use trash::TrashService;
pub use watch::WatchService;
