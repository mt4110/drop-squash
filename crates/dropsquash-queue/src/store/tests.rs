use std::path::PathBuf;

use dropsquash_core::{EncodeJob, OutputSize, Profile, SourcePolicy};

use super::InMemoryQueue;
use crate::{QueueJobId, QueueJobStatus};

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

#[test]
fn starts_jobs_in_enqueue_order() {
    let mut queue = InMemoryQueue::default();
    let first = queue.enqueue(job("first.mov"));
    let second = queue.enqueue(job("second.mov"));

    assert_eq!(queue.start_next().map(|item| item.id), Some(first.id));
    assert!(queue.start_next().is_none());

    queue.finish_active();
    assert_eq!(queue.start_next().map(|item| item.id), Some(second.id));
}

#[test]
fn completed_jobs_keep_terminal_status() {
    let mut queue = InMemoryQueue::default();
    queue.enqueue(job("first.mov"));

    queue.start_next();
    let completed = queue.fail_active("decode failed".to_string()).unwrap();

    assert_eq!(completed.status, QueueJobStatus::Failed);
    assert_eq!(completed.error.as_deref(), Some("decode failed"));
    assert_eq!(queue.completed(), &[completed]);
}

#[test]
fn terminal_active_jobs_release_next_pending_job() {
    let mut queue = InMemoryQueue::default();
    queue.enqueue(job("first.mov"));
    let second = queue.enqueue(job("second.mov"));

    queue.start_next();
    let cancelled = queue.cancel_active().unwrap();

    assert_eq!(cancelled.status, QueueJobStatus::Cancelled);
    assert_eq!(queue.start_next().map(|item| item.id), Some(second.id));
}

#[test]
fn failed_active_job_releases_next_pending_job() {
    let mut queue = InMemoryQueue::default();
    queue.enqueue(job("first.mov"));
    let second = queue.enqueue(job("second.mov"));

    queue.start_next();
    let failed = queue.fail_active("decode failed".to_string()).unwrap();

    assert_eq!(failed.status, QueueJobStatus::Failed);
    assert_eq!(queue.start_next().map(|item| item.id), Some(second.id));
}

#[test]
fn unchanged_active_job_releases_next_pending_job() {
    let mut queue = InMemoryQueue::default();
    queue.enqueue(job("first.mov"));
    let second = queue.enqueue(job("second.mov"));

    queue.start_next();
    let unchanged = queue.unchanged_active("kept original".to_string()).unwrap();

    assert_eq!(unchanged.status, QueueJobStatus::Unchanged);
    assert_eq!(queue.start_next().map(|item| item.id), Some(second.id));
}

#[test]
fn empty_only_when_no_pending_or_active_job_exists() {
    let mut queue = InMemoryQueue::default();
    assert!(queue.is_empty());

    queue.enqueue(job("first.mov"));
    assert!(!queue.is_empty());

    queue.start_next();
    assert!(!queue.is_empty());

    queue.cancel_active();
    assert!(queue.is_empty());
}

#[test]
fn blocks_pending_jobs_without_touching_active_job() {
    let mut queue = InMemoryQueue::default();
    queue.enqueue(job("first.mov"));
    let second = queue.enqueue(job("second.mov"));
    let third = queue.enqueue(job("third.mov"));

    let active = queue.start_next().unwrap();
    let blocked = queue.block_pending("trial locked".to_string());

    assert_eq!(queue.active().map(|item| item.id), Some(active.id));
    assert_eq!(queue.len(), 0);
    assert_eq!(
        blocked.iter().map(|item| item.id).collect::<Vec<_>>(),
        vec![second.id, third.id]
    );
    assert!(blocked
        .iter()
        .all(|item| item.status == QueueJobStatus::Blocked));
    assert!(blocked
        .iter()
        .all(|item| item.error.as_deref() == Some("trial locked")));
    assert_eq!(queue.completed(), blocked.as_slice());
}

#[test]
fn cancels_pending_job_without_touching_active_job() {
    let mut queue = InMemoryQueue::default();
    let first = queue.enqueue(job("first.mov"));
    let second = queue.enqueue(job("second.mov"));
    let third = queue.enqueue(job("third.mov"));

    let active = queue.start_next().unwrap();
    let cancelled = queue.cancel_pending(second.id).unwrap();

    assert_eq!(active.id, first.id);
    assert_eq!(queue.active().map(|item| item.id), Some(first.id));
    assert_eq!(cancelled.status, QueueJobStatus::Cancelled);
    assert_eq!(queue.completed(), &[cancelled]);

    queue.finish_active();
    assert_eq!(queue.start_next().map(|item| item.id), Some(third.id));
}

#[test]
fn cancel_pending_ignores_active_or_unknown_ids() {
    let mut queue = InMemoryQueue::default();
    let active = queue.enqueue(job("first.mov"));
    queue.start_next();

    assert!(queue.cancel_pending(active.id).is_none());
    assert!(queue.cancel_pending(QueueJobId(999)).is_none());
}

#[test]
fn clear_completed_removes_only_terminal_jobs() {
    let mut queue = InMemoryQueue::default();
    queue.enqueue(job("first.mov"));
    let second = queue.enqueue(job("second.mov"));
    let third = queue.enqueue(job("third.mov"));

    let active = queue.start_next().unwrap();
    let cancelled = queue.cancel_pending(second.id).unwrap();

    let cleared = queue.clear_completed();

    assert_eq!(queue.active().map(|item| item.id), Some(active.id));
    assert_eq!(cleared, vec![cancelled]);
    assert!(queue.completed().is_empty());

    queue.finish_active();
    assert_eq!(queue.start_next().map(|item| item.id), Some(third.id));
}
