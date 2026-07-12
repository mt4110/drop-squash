use super::AppState;
use dropsquash_core::{EncodeJob, OutputSize, Profile, SourcePolicy};
use dropsquash_queue::{QueueEvent, QueueJobStatus};

#[test]
fn starts_and_finishes_conversion() {
    let state = AppState::default();

    assert!(state.start_conversion().is_ok());
    assert!(state.start_conversion().is_err());
    state.finish_conversion();
    assert!(state.start_conversion().is_ok());
}

#[test]
fn cancel_marks_active_token() {
    let state = AppState::default();
    let token = state.start_conversion().unwrap();

    assert!(state.cancel_conversion().unwrap());
    assert!(token.is_cancelled());
}

#[test]
fn cancel_without_active_conversion_reports_false() {
    let state = AppState::default();

    assert!(!state.cancel_conversion().unwrap());
}

#[test]
fn owns_rust_queue_state_for_sequential_jobs() {
    let state = AppState::default();
    state.enqueue_job(job("first.mov")).unwrap();
    state.enqueue_job(job("second.mov")).unwrap();

    assert!(matches!(
        state.start_next_job().unwrap(),
        Some(QueueEvent::Started(item)) if item.status == QueueJobStatus::Running
    ));
    let blocked = state
        .block_queued_jobs("Trial complete. Enter a license key to continue.".into())
        .unwrap();

    assert!(matches!(
        blocked.as_slice(),
        [QueueEvent::Blocked { error, .. }]
            if error.contains("Trial complete")
    ));
}

#[test]
fn cancels_pending_rust_queue_job_without_starting_it() {
    let state = AppState::default();
    state.enqueue_job(job("first.mov")).unwrap();
    let queued_id = match state.enqueue_job(job("second.mov")).unwrap() {
        QueueEvent::Enqueued(item) => item.id,
        other => panic!("unexpected event: {other:?}"),
    };

    state.start_next_job().unwrap();

    assert_eq!(
        state.cancel_queued_job(queued_id).unwrap(),
        Some(QueueEvent::Cancelled(queued_id))
    );
}

fn job(path: &str) -> EncodeJob {
    EncodeJob {
        input_path: path.into(),
        output_dir: "/tmp/out".into(),
        profile: Profile::Auto,
        output_size: OutputSize::Auto,
        source_policy: SourcePolicy::Ask,
    }
}
