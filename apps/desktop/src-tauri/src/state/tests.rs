use super::AppState;
use dropsquash_core::{EncodeJob, EncodeResult, OutputSize, Profile, SourcePolicy};
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
        .block_queued_jobs("You used 20 successful conversions.".into())
        .unwrap();

    assert!(matches!(
        blocked.as_slice(),
        [QueueEvent::Blocked { error, .. }]
            if error.contains("20 successful conversions")
    ));
}

#[test]
fn enqueues_multiple_jobs_in_order() {
    let state = AppState::default();
    let events = state
        .enqueue_jobs(vec![job("first.mov"), job("second.mov")])
        .unwrap();

    assert!(matches!(
        events.as_slice(),
        [QueueEvent::Enqueued(first), QueueEvent::Enqueued(second)]
            if first.job.input_path.ends_with("first.mov")
                && second.job.input_path.ends_with("second.mov")
    ));
    assert!(matches!(
        state.start_next_job().unwrap(),
        Some(QueueEvent::Started(item)) if item.job.input_path.ends_with("first.mov")
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

#[test]
fn clears_completed_queue_jobs_without_touching_active_job() {
    let state = AppState::default();
    state.enqueue_job(job("first.mov")).unwrap();
    let queued_id = match state.enqueue_job(job("second.mov")).unwrap() {
        QueueEvent::Enqueued(item) => item.id,
        other => panic!("unexpected event: {other:?}"),
    };

    state.start_next_job().unwrap();
    state.cancel_queued_job(queued_id).unwrap();

    let cleared = state.clear_completed_jobs().unwrap();

    assert_eq!(cleared.len(), 1);
    assert_eq!(cleared[0].id, queued_id);
    assert!(state.start_next_job().unwrap().is_none());
}

#[test]
fn finishes_active_queue_job_and_releases_next_job() {
    let state = AppState::default();
    state.enqueue_job(job("first.mov")).unwrap();
    state.enqueue_job(job("second.mov")).unwrap();
    state.start_next_job().unwrap();

    let finished = state.finish_active_queue_job(result("first.mov")).unwrap();

    assert!(matches!(
        finished,
        Some(QueueEvent::Finished { result, .. }) if result.success
    ));
    assert!(matches!(
        state.start_next_job().unwrap(),
        Some(QueueEvent::Started(item)) if item.job.input_path.ends_with("second.mov")
    ));
}

#[test]
fn fails_or_cancels_active_queue_job_without_touching_pending() {
    let state = AppState::default();
    state.enqueue_job(job("first.mov")).unwrap();
    state.enqueue_job(job("second.mov")).unwrap();
    state.start_next_job().unwrap();

    let failed = state.fail_active_queue_job("decode failed".into()).unwrap();

    assert!(matches!(
        failed,
        Some(QueueEvent::Failed { error, .. }) if error == "decode failed"
    ));
    assert!(matches!(
        state.start_next_job().unwrap(),
        Some(QueueEvent::Started(_))
    ));
    assert!(matches!(
        state.cancel_active_queue_job().unwrap(),
        Some(QueueEvent::Cancelled(_))
    ));
}

#[test]
fn marks_kept_original_queue_job_without_touching_pending() {
    let state = AppState::default();
    state.enqueue_job(job("first.mov")).unwrap();
    state.enqueue_job(job("second.mov")).unwrap();
    state.start_next_job().unwrap();

    let unchanged = state
        .unchanged_active_queue_job("kept original".into())
        .unwrap();

    assert!(matches!(
        unchanged,
        Some(QueueEvent::Unchanged { error, .. }) if error == "kept original"
    ));
    assert!(matches!(
        state.start_next_job().unwrap(),
        Some(QueueEvent::Started(_))
    ));
}

fn job(path: &str) -> EncodeJob {
    EncodeJob {
        input_path: path.into(),
        output_dir: "/tmp/out".into(),
        profile: Profile::Auto,
        output_size: OutputSize::Auto,
        source_policy: SourcePolicy::Ask,
        secure_share: None,
    }
}

fn result(path: &str) -> EncodeResult {
    EncodeResult {
        input_path: path.into(),
        output_path: "/tmp/out/out.mp4".into(),
        profile: Profile::Auto,
        original_bytes: 100,
        output_bytes: 50,
        success: true,
        error_message: None,
    }
}
