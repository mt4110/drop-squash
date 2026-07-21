use std::path::PathBuf;

use dropsquash_core::{EncodeJob, EncodeResult, OutputSize, Profile, SourcePolicy};

use crate::{QueueEvent, QueueJobStatus, QueueWorker};

fn job(path: &str) -> EncodeJob {
    EncodeJob {
        input_path: PathBuf::from(path),
        output_dir: PathBuf::from("/tmp/out"),
        profile: Profile::Auto,
        output_size: OutputSize::Auto,
        source_policy: SourcePolicy::Ask,
        secure_share: None,
    }
}

fn result(path: &str) -> EncodeResult {
    EncodeResult {
        input_path: PathBuf::from(path),
        output_path: PathBuf::from("/tmp/out/out.mp4"),
        profile: Profile::Auto,
        original_bytes: 100,
        output_bytes: 50,
        success: true,
        error_message: None,
    }
}

#[test]
fn emits_events_for_sequential_success() {
    let mut worker = QueueWorker::default();
    let enqueued = worker.enqueue(job("first.mov"));
    let id = match enqueued {
        QueueEvent::Enqueued(item) => item.id,
        other => panic!("unexpected event: {other:?}"),
    };

    assert!(matches!(
        worker.start_next(),
        Some(QueueEvent::Started(item))
            if item.id == id && item.status == QueueJobStatus::Running
    ));
    assert!(matches!(
        worker.finish_active(result("first.mov")),
        Some(QueueEvent::Finished { id: finished_id, result }) if finished_id == id && result.success
    ));
}

#[test]
fn cancels_pending_without_starting_it() {
    let mut worker = QueueWorker::default();
    worker.enqueue(job("first.mov"));
    let queued = match worker.enqueue(job("second.mov")) {
        QueueEvent::Enqueued(item) => item.id,
        other => panic!("unexpected event: {other:?}"),
    };

    worker.start_next();

    assert_eq!(
        worker.cancel_pending(queued),
        Some(QueueEvent::Cancelled(queued))
    );
}

#[test]
fn emits_blocked_events_for_pending_jobs() {
    let mut worker = QueueWorker::default();
    worker.enqueue(job("first.mov"));
    worker.enqueue(job("second.mov"));
    worker.start_next();

    let events = worker.block_pending("trial locked".to_string());

    assert!(matches!(
        events.as_slice(),
        [QueueEvent::Blocked { error, .. }] if error == "trial locked"
    ));
}

#[test]
fn clears_completed_items_without_touching_active_job() {
    let mut worker = QueueWorker::default();
    worker.enqueue(job("first.mov"));
    let queued = match worker.enqueue(job("second.mov")) {
        QueueEvent::Enqueued(item) => item.id,
        other => panic!("unexpected event: {other:?}"),
    };

    worker.start_next();
    worker.cancel_pending(queued);

    let cleared = worker.clear_completed();

    assert_eq!(cleared.len(), 1);
    assert_eq!(cleared[0].id, queued);
    assert!(worker.active().is_some());
}

#[test]
fn emits_unchanged_event_for_kept_original_result() {
    let mut worker = QueueWorker::default();
    worker.enqueue(job("first.mov"));
    let id = match worker.start_next() {
        Some(QueueEvent::Started(item)) => item.id,
        other => panic!("unexpected event: {other:?}"),
    };

    assert_eq!(
        worker.unchanged_active("kept original".to_string()),
        Some(QueueEvent::Unchanged {
            id,
            error: "kept original".to_string(),
        })
    );
}
