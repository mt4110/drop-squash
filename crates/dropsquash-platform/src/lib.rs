mod dialog;
mod install;
mod keychain;
mod notify;
mod secure_share;
mod trash;
mod watch;

pub use dialog::DialogService;
pub use install::{
    copy_app_bundle, copy_current_app_to_applications, current_install_location,
    eject_mounted_volume, install_location_for_executable, open_installed_application,
    reveal_finder_item, validate_installed_app_path, validate_mounted_volume_path,
    ApplicationsInstall, InstallLocation, InstallerCleanup,
};
pub use keychain::KeychainStore;
pub use notify::NotificationService;
pub use secure_share::{
    build_stream_capture_plan, capture_snapshot_from_providers, core_media_attachment_binding_name,
    ensure_required_sck_frame_attachments, make_text_request, make_text_shape_request,
    observe_strict_reveal_window_once, observe_window_once, observe_window_once_callback,
    perform_text_requests, raw_sck_frame_info_from_sample_buffer,
    request_shareable_content_snapshot, sample_buffer_request_handler,
    screen_capture_kit_binding_name, select_explicit_display_target, select_explicit_window_target,
    select_strict_reveal_window_target, snapshot_shareable_content, text_observations,
    text_shape_observations, vision_binding_name, AccessibilityObservationProvider,
    FrameMetadataProvider, NativeFrameInfoKey, NativeFrameMetadataProvider, NativeSampleBuffer,
    NativeSampleBufferProvider, NativeVisionImageOption, NativeVisionImageRequestHandler,
    NativeVisionObservationProvider, NativeVisionRequest, NativeVisionRequests,
    NativeVisionTextRequest, NativeVisionTextShapeRequest, RawSckFrameInfo, RawSckFrameStatus,
    SampleBufferFrameMetadataProvider, SckCaptureTarget, SckCaptureTargetKind, SckDisplayCandidate,
    SckFrameAttachmentPresence, SckFrameMetadataStreamRegistration, SckLiveMaskEvidence,
    SckObservationProbeReport, SckObservationProbeRequest, SckShareableContentRequest,
    SckShareableContentSnapshot, SckStreamCapturePlan, SckStreamFrameMetadataOutput,
    SckWindowCandidate, SecureShareObservationSnapshot, SecureShareProbe,
    SecureShareSnapshotProvider, VisionObservationProvider,
};
pub use trash::TrashService;
pub use watch::WatchService;
