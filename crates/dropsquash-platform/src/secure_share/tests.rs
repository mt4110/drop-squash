use dropsquash_core::{
    AxObservation, AxObservationKind, CaptureFrameMetadata, CaptureRect, Confidence, FrameSize,
    FrameStatus, MaskPolicy, MaskReason, PixelRect, TimeRangeNs, VerificationExpectations,
    VisionObservation,
};

#[cfg(target_os = "macos")]
use super::shareable_request::request_shareable_content_snapshot_with;
#[cfg(target_os = "macos")]
use super::stream_lifecycle::wait_for_stream_completion;
use super::{
    capture_snapshot_from_providers, core_media_attachment_binding_name,
    ensure_required_sck_frame_attachments, screen_capture_kit_binding_name,
    AccessibilityObservationProvider, FrameMetadataProvider, NativeFrameMetadataProvider,
    RawSckFrameInfo, RawSckFrameStatus, SckFrameAttachmentPresence, SecureShareObservationSnapshot,
    SecureShareProbe, SecureShareSnapshotProvider, VisionObservationProvider,
};
#[cfg(target_os = "macos")]
use super::{make_text_request, make_text_shape_request, native_vision_result};
#[cfg(target_os = "macos")]
use super::{
    select_explicit_display_target, select_strict_reveal_window_target, NativeSampleBuffer,
    NativeSampleBufferProvider, NativeVisionObservationProvider, SampleBufferFrameMetadataProvider,
    SckCaptureTargetKind, SckDisplayCandidate, SckShareableContentRequest,
    SckShareableContentSnapshot, SckStreamFrameMetadataOutput, SckWindowCandidate,
};
#[cfg(target_os = "macos")]
use objc2_core_foundation::{CGPoint, CGRect, CGSize};
#[cfg(target_os = "macos")]
use objc2_foundation::NSArray;
#[cfg(target_os = "macos")]
use std::time::Duration;

#[test]
fn secure_share_probe_reports_unimplemented_boundary() {
    let error = SecureShareProbe
        .capture_snapshot()
        .expect_err("probe should not silently succeed yet")
        .to_string();

    assert!(error.contains("Secure Share"));
    assert!(error.contains("frame metadata"));
}

#[test]
fn native_frame_metadata_provider_fails_closed_until_sck_exists() {
    let error = NativeFrameMetadataProvider
        .frame_size()
        .expect_err("native frame metadata is not wired yet")
        .to_string();

    assert!(error.contains("Secure Share"));
    assert!(error.contains("frame metadata"));
}

#[test]
fn native_frame_metadata_provider_names_sck_binding() {
    assert_eq!(
        screen_capture_kit_binding_name(),
        "objc2-screen-capture-kit"
    );
}

#[test]
fn native_attachment_extractor_names_core_media_binding() {
    assert_eq!(core_media_attachment_binding_name(), "objc2-core-media");
}

#[cfg(target_os = "macos")]
#[test]
fn native_vision_provider_names_vision_binding() {
    assert_eq!(super::vision_binding_name(), "objc2-vision");
}

#[cfg(target_os = "macos")]
#[test]
fn native_vision_provider_fails_closed_until_frame_pixels_exist() {
    let error = NativeVisionObservationProvider
        .vision_observations()
        .expect_err("Vision needs captured pixel buffers")
        .to_string();

    assert!(error.contains("Vision observation"));
    assert!(error.contains("pixel buffers"));
}

#[cfg(target_os = "macos")]
#[test]
fn native_vision_text_request_is_configured_for_ja_and_en_without_text_storage() {
    let request = make_text_request();
    let languages = unsafe { request.recognitionLanguages() };

    assert_eq!(languages.count(), 2);
    assert_eq!(languages.objectAtIndex(0).to_string(), "ja-JP");
    assert_eq!(languages.objectAtIndex(1).to_string(), "en-US");
    assert!(!request.usesLanguageCorrection());
}

#[cfg(target_os = "macos")]
#[test]
fn native_vision_text_shape_request_omits_character_boxes() {
    let request = make_text_shape_request();

    assert!(!unsafe { request.reportCharacterBoxes() });
}

#[cfg(target_os = "macos")]
#[test]
fn native_vision_text_shapes_map_to_redacted_observations() {
    let rect = CGRect::new(CGPoint::new(0.25, 0.50), CGSize::new(0.25, 0.25));
    let result = unsafe { objc2_vision::VNTextObservation::observationWithBoundingBox(rect) };
    let results = NSArray::from_retained_slice(&[result]);
    let observations = native_vision_result::observations_from_text_shapes(
        &results,
        FrameSize {
            width: 800,
            height: 600,
        },
        TimeRangeNs {
            start_ns: 10,
            end_ns: 20,
        },
    );

    assert_eq!(observations.len(), 1);
    assert_eq!(
        observations[0].rect,
        PixelRect {
            x: 200,
            y: 150,
            width: 200,
            height: 150
        }
    );
    assert_eq!(observations[0].time_range.start_ns, 10);
}

#[test]
fn sck_frame_attachment_presence_accepts_required_keys() {
    ensure_required_sck_frame_attachments(SckFrameAttachmentPresence::complete()).unwrap();
}

#[test]
fn sck_frame_attachment_presence_rejects_missing_key() {
    let mut presence = SckFrameAttachmentPresence::complete();
    presence.bounding_rect = false;

    let error = ensure_required_sck_frame_attachments(presence)
        .expect_err("missing metadata key must fail closed")
        .to_string();

    assert!(error.contains("missing"));
    assert!(error.contains("bounding rect"));
}

#[test]
fn raw_sck_frame_info_maps_to_capture_metadata() {
    let metadata = sample_raw_sck_frame_info().into_metadata().unwrap();

    assert_eq!(metadata.frame_index, 7);
    assert_eq!(metadata.presentation_time_ns, 900);
    assert_eq!(metadata.frame_status, FrameStatus::Complete);
    assert_eq!(metadata.content_rect.width, 640);
    assert_eq!(metadata.bounding_rect.height, 720);
    assert_eq!(metadata.scale_factor, 2.0);
}

#[test]
fn raw_sck_frame_info_rejects_empty_rects() {
    let mut info = sample_raw_sck_frame_info();
    info.content_rect.width = 0;

    let error = info.into_metadata().unwrap_err().to_string();

    assert!(error.contains("content rect"));
    assert!(error.contains("empty"));
}

#[test]
fn raw_sck_frame_info_rejects_invalid_scale() {
    let mut info = sample_raw_sck_frame_info();
    info.content_scale = f32::NAN;

    let error = info.into_metadata().unwrap_err().to_string();

    assert!(error.contains("content scale"));
    assert!(error.contains("invalid"));
}

#[test]
fn snapshot_builds_strict_reveal_mask_plan() {
    let plan = sample_snapshot().into_mask_plan(
        "capture-platform-001".to_string(),
        MaskPolicy::StrictReveal,
        verification(),
    );

    assert_eq!(plan.capture_id, "capture-platform-001");
    assert_eq!(plan.frames.len(), 1);
    assert_eq!(plan.frames[0].regions[0].reason, MaskReason::AxTextElement);
}

#[test]
fn probe_accepts_injected_snapshot_provider() {
    let snapshot = SecureShareProbe
        .capture_snapshot_with(&FakeSnapshotProvider)
        .unwrap();

    let plan = snapshot.into_mask_plan(
        "capture-provider-001".to_string(),
        MaskPolicy::StrictReveal,
        verification(),
    );

    assert_eq!(plan.capture_id, "capture-provider-001");
    assert_eq!(plan.frames.len(), 1);
    assert_eq!(plan.frames[0].regions[0].reason, MaskReason::AxTextElement);
}

#[test]
fn snapshot_can_be_composed_from_separate_signal_providers() {
    let snapshot =
        capture_snapshot_from_providers(&FakeFrames, &FakeAccessibility, &FakeVision).unwrap();

    let plan = snapshot.into_mask_plan(
        "capture-composed-001".to_string(),
        MaskPolicy::StrictReveal,
        verification(),
    );

    assert_eq!(plan.frames.len(), 1);
    assert_eq!(plan.frames[0].regions.len(), 2);
    assert!(plan.frames[0]
        .regions
        .iter()
        .any(|region| region.reason == MaskReason::AxTextElement));
    assert!(plan.frames[0]
        .regions
        .iter()
        .any(|region| region.reason == MaskReason::VisionText));
}

#[cfg(target_os = "macos")]
#[test]
fn sample_buffer_frame_provider_rejects_empty_stream() {
    let provider = SampleBufferFrameMetadataProvider::new(EmptySampleBuffers);
    let error = provider
        .frames()
        .expect_err("empty sample buffer stream must fail closed")
        .to_string();

    assert!(error.contains("ScreenCaptureKit"));
    assert!(error.contains("no frame metadata"));
}

#[cfg(target_os = "macos")]
#[test]
fn sck_stream_output_exposes_protocol_and_rejects_empty_frames() {
    let output = SckStreamFrameMetadataOutput::new(sample_frame_size());
    let _protocol = output.as_stream_output();
    let error = output
        .frames()
        .expect_err("stream output without captured frames must fail closed")
        .to_string();

    assert_eq!(output.frame_size().unwrap(), sample_frame_size());
    assert!(error.contains("ScreenCaptureKit"));
    assert!(error.contains("no frame metadata"));
}

#[cfg(target_os = "macos")]
#[test]
fn strict_reveal_policy_selects_one_active_owned_window() {
    let target = select_strict_reveal_window_target(&sample_shareable_snapshot()).unwrap();

    assert_eq!(target.kind, SckCaptureTargetKind::Window);
    assert_eq!(target.id, 42);
    assert_eq!(target.frame.width, 800);
}

#[cfg(target_os = "macos")]
#[test]
fn strict_reveal_policy_rejects_ambiguous_windows() {
    let mut snapshot = sample_shareable_snapshot();
    snapshot.windows.push(sample_window(43));
    let error = select_strict_reveal_window_target(&snapshot)
        .expect_err("ambiguous target selection must fail closed")
        .to_string();

    assert!(error.contains("target selection"));
    assert!(error.contains("multiple"));
}

#[cfg(target_os = "macos")]
#[test]
fn strict_reveal_policy_rejects_ineligible_windows() {
    let mut snapshot = sample_shareable_snapshot();
    snapshot.windows[0].active = false;
    let error = select_strict_reveal_window_target(&snapshot)
        .expect_err("inactive target selection must fail closed")
        .to_string();

    assert!(error.contains("target selection"));
    assert!(error.contains("no eligible"));
}

#[cfg(target_os = "macos")]
#[test]
fn explicit_display_policy_requires_matching_display() {
    let snapshot = sample_shareable_snapshot();
    let target = select_explicit_display_target(&snapshot, 7).unwrap();
    let error = select_explicit_display_target(&snapshot, 99)
        .expect_err("unknown display must fail closed")
        .to_string();

    assert_eq!(target.kind, SckCaptureTargetKind::Display);
    assert_eq!(target.id, 7);
    assert!(error.contains("display"));
}

#[cfg(target_os = "macos")]
#[test]
fn shareable_content_request_rejects_missing_content() {
    let error =
        request_shareable_content_snapshot_with(fast_shareable_request(), |_request, done| {
            done.call((std::ptr::null_mut(), std::ptr::null_mut()));
        })
        .expect_err("missing shareable content must fail closed")
        .to_string();

    assert!(error.contains("ScreenCaptureKit"));
    assert!(error.contains("no shareable content"));
}

#[cfg(target_os = "macos")]
#[test]
fn shareable_content_request_times_out_without_callback() {
    let error =
        request_shareable_content_snapshot_with(fast_shareable_request(), |_request, done| {
            std::mem::forget(done);
        })
        .expect_err("missing callback must fail closed")
        .to_string();

    assert!(error.contains("ScreenCaptureKit"));
    assert!(error.contains("timed out"));
}

#[cfg(target_os = "macos")]
#[test]
fn stream_lifecycle_accepts_successful_completion() {
    wait_for_stream_completion("start", fast_timeout(), |done| {
        done.call((std::ptr::null_mut(),));
    })
    .unwrap();
}

#[cfg(target_os = "macos")]
#[test]
fn stream_lifecycle_times_out_without_callback() {
    let error = wait_for_stream_completion("start", fast_timeout(), |done| {
        std::mem::forget(done);
    })
    .expect_err("missing callback must fail closed")
    .to_string();

    assert!(error.contains("ScreenCaptureKit"));
    assert!(error.contains("start timed out"));
}

#[cfg(target_os = "macos")]
#[test]
fn ax_observation_maps_point_size_without_private_text() {
    let observation = super::ax_rect::observation_from_point_size(
        CGPoint { x: 12.4, y: 34.6 },
        CGSize {
            width: 200.2,
            height: 80.8,
        },
        sample_time_range(),
    )
    .unwrap();

    assert_eq!(observation.rect.x, 12);
    assert_eq!(observation.rect.y, 35);
    assert_eq!(observation.rect.width, 200);
    assert_eq!(observation.kind, AxObservationKind::UnknownClientArea);
}

struct FakeSnapshotProvider;

impl SecureShareSnapshotProvider for FakeSnapshotProvider {
    fn capture_snapshot(&self) -> dropsquash_core::Result<SecureShareObservationSnapshot> {
        Ok(sample_snapshot())
    }
}

struct FakeFrames;

impl FrameMetadataProvider for FakeFrames {
    fn frame_size(&self) -> dropsquash_core::Result<FrameSize> {
        Ok(sample_frame_size())
    }

    fn frames(&self) -> dropsquash_core::Result<Vec<CaptureFrameMetadata>> {
        Ok(sample_frames())
    }
}

struct FakeAccessibility;

impl AccessibilityObservationProvider for FakeAccessibility {
    fn accessibility_observations(&self) -> dropsquash_core::Result<Vec<AxObservation>> {
        Ok(sample_accessibility())
    }
}

struct FakeVision;

impl VisionObservationProvider for FakeVision {
    fn vision_observations(&self) -> dropsquash_core::Result<Vec<VisionObservation>> {
        Ok(vec![VisionObservation {
            rect: PixelRect {
                x: 80,
                y: 120,
                width: 360,
                height: 28,
            },
            time_range: sample_time_range(),
            kind: dropsquash_core::VisionObservationKind::TextRecognition,
            confidence: Confidence::CERTAIN,
        }])
    }
}

#[cfg(target_os = "macos")]
struct EmptySampleBuffers;

#[cfg(target_os = "macos")]
impl NativeSampleBufferProvider for EmptySampleBuffers {
    fn frame_size(&self) -> dropsquash_core::Result<FrameSize> {
        Ok(sample_frame_size())
    }

    fn visit_sample_buffers(
        &self,
        _visitor: &mut dyn FnMut(&NativeSampleBuffer) -> dropsquash_core::Result<()>,
    ) -> dropsquash_core::Result<()> {
        Ok(())
    }
}

fn sample_snapshot() -> SecureShareObservationSnapshot {
    SecureShareObservationSnapshot {
        frame_size: sample_frame_size(),
        frames: sample_frames(),
        accessibility: sample_accessibility(),
        vision: Vec::new(),
    }
}

fn sample_frame_size() -> FrameSize {
    FrameSize {
        width: 1280,
        height: 720,
    }
}

fn sample_frames() -> Vec<CaptureFrameMetadata> {
    vec![CaptureFrameMetadata {
        frame_index: 0,
        presentation_time_ns: 0,
        frame_status: FrameStatus::Complete,
        content_rect: CaptureRect {
            x: 0,
            y: 0,
            width: 1280,
            height: 720,
        },
        bounding_rect: CaptureRect {
            x: 0,
            y: 0,
            width: 1280,
            height: 720,
        },
        scale_factor: 1.0,
        content_scale: 1.0,
    }]
}

fn sample_accessibility() -> Vec<AxObservation> {
    vec![AxObservation {
        rect: PixelRect {
            x: 20,
            y: 40,
            width: 200,
            height: 32,
        },
        time_range: sample_time_range(),
        kind: AxObservationKind::TextElement,
        confidence: Confidence::CERTAIN,
    }]
}

fn sample_time_range() -> TimeRangeNs {
    TimeRangeNs {
        start_ns: 0,
        end_ns: 1,
    }
}

#[cfg(target_os = "macos")]
fn sample_shareable_snapshot() -> SckShareableContentSnapshot {
    SckShareableContentSnapshot {
        windows: vec![sample_window(42)],
        displays: vec![SckDisplayCandidate {
            display_id: 7,
            width_points: 1440,
            height_points: 900,
            frame: CaptureRect {
                x: 0,
                y: 0,
                width: 1440,
                height: 900,
            },
        }],
    }
}

#[cfg(target_os = "macos")]
fn fast_shareable_request() -> SckShareableContentRequest {
    SckShareableContentRequest {
        exclude_desktop_windows: true,
        on_screen_windows_only: true,
        timeout: fast_timeout(),
    }
}

#[cfg(target_os = "macos")]
fn fast_timeout() -> Duration {
    Duration::from_millis(1)
}

#[cfg(target_os = "macos")]
fn sample_window(window_id: u32) -> SckWindowCandidate {
    SckWindowCandidate {
        window_id,
        title: Some("Fixture".to_string()),
        frame: CaptureRect {
            x: 20,
            y: 40,
            width: 800,
            height: 600,
        },
        layer: 0,
        on_screen: true,
        active: true,
        has_title: true,
        has_owner: true,
        owner_pid: Some(1234),
    }
}

fn sample_raw_sck_frame_info() -> RawSckFrameInfo {
    RawSckFrameInfo {
        frame_index: 7,
        presentation_time_ns: 900,
        status: RawSckFrameStatus::Complete,
        content_rect: CaptureRect {
            x: 10,
            y: 20,
            width: 640,
            height: 360,
        },
        bounding_rect: CaptureRect {
            x: 0,
            y: 0,
            width: 1280,
            height: 720,
        },
        scale_factor: 2.0,
        content_scale: 1.0,
    }
}

fn verification() -> VerificationExpectations {
    VerificationExpectations {
        no_audio: true,
        strip_metadata: true,
        verification_policy_version: "secure-share-rd-1".to_string(),
    }
}
