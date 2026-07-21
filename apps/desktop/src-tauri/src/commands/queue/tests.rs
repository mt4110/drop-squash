use dropsquash_core::{MaskMode, MaskRect, OutputSize, Profile, SecureShareOptions, SourcePolicy};
use dropsquash_queue::{QueueEvent, QueueJobStatus};

use super::{
    block_pending_jobs, fail_or_unchanged_active_queue_job, job_from_request, normalize_queue_error,
};
use crate::commands::dto::ConvertRequest;
use crate::state::AppState;

#[test]
fn queue_job_uses_conversion_request_settings() {
    let job = job_from_request(ConvertRequest {
        input_path: "/tmp/in.mov".into(),
        output_dir: "/tmp/out".into(),
        profile: Profile::Docs,
        output_size: OutputSize::P720,
        source_policy: SourcePolicy::Ask,
        write_privacy_receipt: true,
        secure_share: None,
    });

    assert_eq!(job.input_path, std::path::Path::new("/tmp/in.mov"));
    assert_eq!(job.output_dir, std::path::Path::new("/tmp/out"));
    assert_eq!(job.profile, Profile::Docs);
    assert_eq!(job.output_size, OutputSize::P720);
    assert_eq!(job.source_policy, SourcePolicy::Ask);
}

#[test]
fn queue_job_preserves_multiple_secure_share_rects() {
    let job = job_from_request(ConvertRequest {
        input_path: "/tmp/in.mov".into(),
        output_dir: "/tmp/out".into(),
        profile: Profile::Docs,
        output_size: OutputSize::P720,
        source_policy: SourcePolicy::Ask,
        write_privacy_receipt: true,
        secure_share: Some(SecureShareOptions {
            mask_mode: MaskMode::SolidBlack,
            mask_rects: vec![
                MaskRect {
                    x: 10,
                    y: 20,
                    width: 300,
                    height: 60,
                },
                MaskRect {
                    x: 40,
                    y: 120,
                    width: 280,
                    height: 80,
                },
            ],
            mask_plan: None,
        }),
    });

    let secure_share = job.secure_share.expect("secure share should exist");
    assert_eq!(secure_share.mask_mode, MaskMode::SolidBlack);
    assert_eq!(secure_share.mask_rects.len(), 2);
    assert_eq!(secure_share.mask_rects[1].y, 120);
}

#[test]
fn blocks_remaining_queue_jobs_with_license_lock_reason() {
    let state = AppState::default();
    state.enqueue_job(request_job("first.mov")).unwrap();
    state.enqueue_job(request_job("second.mov")).unwrap();

    assert!(matches!(
        state.start_next_job().unwrap(),
        Some(QueueEvent::Started(item)) if item.status == QueueJobStatus::Running
    ));

    let events = block_pending_jobs(&state, "You used 20 successful conversions.".into()).unwrap();

    assert!(matches!(
        events.as_slice(),
        [QueueEvent::Blocked { error, .. }] if error.contains("20 successful conversions")
    ));
}

#[test]
fn blocks_remaining_queue_jobs_with_refresh_lock_reason() {
    let state = AppState::default();
    state.enqueue_job(request_job("first.mov")).unwrap();
    state.enqueue_job(request_job("second.mov")).unwrap();
    state.start_next_job().unwrap();

    let message = "Reconnect once with your license key to refresh Pro.";
    let events = block_pending_jobs(&state, message.into()).unwrap();

    assert!(matches!(
        events.as_slice(),
        [QueueEvent::Blocked { error, .. }] if error == message
    ));
}

#[test]
fn rewrites_not_smaller_failures_before_queue_storage() {
    assert_eq!(
        normalize_queue_error(
        "native export failed output verification: output is not smaller (1 bytes -> 2 bytes)"
            .into(),
        ),
        "This recording could not be made smaller. It may already be small, so DropSquash kept the original and did not count the attempt. Try a smaller Size setting for this clip."
    );
}

#[test]
fn stores_not_smaller_failures_as_unchanged() {
    let state = AppState::default();
    state.enqueue_job(request_job("first.mov")).unwrap();
    state.start_next_job().unwrap();

    let event = fail_or_unchanged_active_queue_job(
        &state,
        "native export failed output verification: output is not smaller (1 bytes -> 2 bytes)"
            .into(),
    )
    .unwrap();

    assert!(matches!(event, Some(QueueEvent::Unchanged { .. })));
}

fn request_job(file_name: &str) -> dropsquash_core::EncodeJob {
    job_from_request(ConvertRequest {
        input_path: format!("/tmp/{file_name}"),
        output_dir: "/tmp/out".into(),
        profile: Profile::Auto,
        output_size: OutputSize::Auto,
        source_policy: SourcePolicy::Ask,
        write_privacy_receipt: false,
        secure_share: None,
    })
}
