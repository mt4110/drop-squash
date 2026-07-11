use std::path::PathBuf;

use dropsquash_core::{EncodeJob, OutputSize, Profile, SourcePolicy};

use super::InMemoryQueue;
use crate::QueueJobStatus;

fn job(path: &str) -> EncodeJob {
    EncodeJob {
        input_path: PathBuf::from(path),
        output_dir: PathBuf::from("/tmp/out"),
        profile: Profile::Auto,
        output_size: OutputSize::Auto,
        source_policy: SourcePolicy::Ask,
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
