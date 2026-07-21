use std::thread;

use super::*;

#[test]
fn cancelled_recording_cannot_publish_late_output() {
    let (_directory, paths) = paths();
    let temporary = paths.final_path.with_extension("mask-plan.json.partial");
    std::fs::write(&temporary, b"receipt").unwrap();
    let permit = PublicationPermit::default();

    permit.cancel(&paths);

    assert!(permit.publish(&temporary, &paths).is_err());
    assert_no_artifacts(&paths, &temporary);
}

#[test]
fn failed_video_publication_removes_the_published_sidecar() {
    let (_directory, paths) = paths();
    std::fs::create_dir(&paths.final_path).unwrap();
    let temporary = paths.final_path.with_extension("mask-plan.json.partial");
    std::fs::write(&temporary, b"receipt").unwrap();

    assert!(PublicationPermit::default()
        .publish(&temporary, &paths)
        .is_err());
    assert!(!paths.final_path.with_extension("mask-plan.json").exists());
}

#[test]
fn poisoned_permit_still_discards_artifacts() {
    let (_directory, paths) = paths();
    let temporary = paths.final_path.with_extension("mask-plan.json.partial");
    std::fs::write(&temporary, b"receipt").unwrap();
    let permit = PublicationPermit::default();
    let poison = permit.clone();
    let _ = thread::spawn(move || {
        let _guard = poison.0.lock().unwrap();
        panic!("poison permit");
    })
    .join();

    assert!(permit.publish(&temporary, &paths).is_err());
    assert_no_artifacts(&paths, &temporary);
}

fn paths() -> (tempfile::TempDir, RecordingPaths) {
    let directory = tempfile::tempdir().unwrap();
    let paths = RecordingPaths {
        partial: directory.path().join("recording.partial.mp4"),
        final_path: directory.path().join("recording.mp4"),
    };
    std::fs::write(&paths.partial, b"partial").unwrap();
    (directory, paths)
}

fn assert_no_artifacts(paths: &RecordingPaths, temporary: &std::path::Path) {
    assert!(!paths.partial.exists());
    assert!(!paths.final_path.exists());
    assert!(!temporary.exists());
}
