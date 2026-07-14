use dropsquash_core::{OutputSize, Profile, SourcePolicy};
use dropsquash_queue::{QueueEvent, QueueJobStatus};

use super::{block_pending_jobs, job_from_request};
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
    });

    assert_eq!(job.input_path, std::path::Path::new("/tmp/in.mov"));
    assert_eq!(job.output_dir, std::path::Path::new("/tmp/out"));
    assert_eq!(job.profile, Profile::Docs);
    assert_eq!(job.output_size, OutputSize::P720);
    assert_eq!(job.source_policy, SourcePolicy::Ask);
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

    let events = block_pending_jobs(&state, "You used 10 successful conversions.".into()).unwrap();

    assert!(matches!(
        events.as_slice(),
        [QueueEvent::Blocked { error, .. }] if error.contains("10 successful conversions")
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

fn request_job(file_name: &str) -> dropsquash_core::EncodeJob {
    job_from_request(ConvertRequest {
        input_path: format!("/tmp/{file_name}"),
        output_dir: "/tmp/out".into(),
        profile: Profile::Auto,
        output_size: OutputSize::Auto,
        source_policy: SourcePolicy::Ask,
        write_privacy_receipt: false,
    })
}
