use std::path::Path;

use super::TrashService;

#[test]
fn missing_source_is_rejected_before_platform_trash_call() {
    let error = TrashService
        .move_to_trash(Path::new("/tmp/dropsquash-missing-source.mov"))
        .unwrap_err();

    assert!(error.to_string().contains("source file does not exist"));
}
