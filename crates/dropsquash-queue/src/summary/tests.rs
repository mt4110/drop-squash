use std::path::PathBuf;

use dropsquash_core::{EncodeJob, EncodeResult, OutputSize, Profile, SourcePolicy};

use crate::{QueueEvent, QueueItem, QueueJobId, QueueSummary};

fn item(id: u64) -> QueueItem {
    QueueItem::queued(QueueJobId(id), job())
}

fn job() -> EncodeJob {
    EncodeJob {
        input_path: PathBuf::from("input.mov"),
        output_dir: PathBuf::from("/tmp/out"),
        profile: Profile::Auto,
        output_size: OutputSize::Auto,
        source_policy: SourcePolicy::Keep,
        secure_share: None,
    }
}

fn result(original_bytes: u64, output_bytes: u64, success: bool) -> EncodeResult {
    EncodeResult {
        input_path: PathBuf::from("input.mov"),
        output_path: PathBuf::from("/tmp/out/input.squashed.mp4"),
        profile: Profile::Auto,
        original_bytes,
        output_bytes,
        success,
        error_message: None,
    }
}

#[test]
fn summarizes_batch_events() {
    let events = vec![
        QueueEvent::Enqueued(item(1)),
        QueueEvent::Enqueued(item(2)),
        QueueEvent::Enqueued(item(3)),
        QueueEvent::Enqueued(item(4)),
        QueueEvent::Finished {
            id: QueueJobId(1),
            result: result(100, 40, true),
        },
        QueueEvent::Failed {
            id: QueueJobId(2),
            error: "decode failed".to_string(),
        },
        QueueEvent::Unchanged {
            id: QueueJobId(3),
            error: "kept original".to_string(),
        },
        QueueEvent::Cancelled(QueueJobId(4)),
        QueueEvent::Blocked {
            id: QueueJobId(5),
            error: "trial locked".to_string(),
        },
    ];

    let summary = QueueSummary::from_events(&events);

    assert_eq!(summary.total, 4);
    assert_eq!(summary.finished, 5);
    assert_eq!(summary.succeeded, 1);
    assert_eq!(summary.unchanged, 1);
    assert_eq!(summary.failed, 1);
    assert_eq!(summary.cancelled, 1);
    assert_eq!(summary.blocked, 1);
    assert_eq!(summary.saved_bytes, 60);
}

#[test]
fn ignores_unsuccessful_or_larger_finished_results_for_saved_bytes() {
    let events = vec![
        QueueEvent::Finished {
            id: QueueJobId(1),
            result: result(100, 120, true),
        },
        QueueEvent::Finished {
            id: QueueJobId(2),
            result: result(100, 50, false),
        },
    ];

    let summary = QueueSummary::from_events(&events);

    assert_eq!(summary.finished, 2);
    assert_eq!(summary.succeeded, 0);
    assert_eq!(summary.failed, 2);
    assert_eq!(summary.saved_bytes, 0);
}
