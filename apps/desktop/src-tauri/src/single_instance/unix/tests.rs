use super::{lock_name_for, prepare_at};

#[test]
fn second_prepare_yields_none_for_existing_instance() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("dropsquash.lock");
    let first = prepare_at(&path).unwrap();
    assert!(first.is_some());
    let second = prepare_at(&path).unwrap();
    assert!(second.is_none());
    drop(first);
}

#[test]
fn existing_lock_file_can_be_locked() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("dropsquash.lock");
    std::fs::write(&path, b"lock").unwrap();
    let guard = prepare_at(&path).unwrap();
    assert!(guard.is_some());
}

#[test]
fn qa_instance_id_uses_separate_safe_lock_name() {
    assert_eq!(
        lock_name_for(Some("sck_observe")),
        "io.github.mt4110.dropsquash.sck_observe.lock"
    );
}

#[test]
fn qa_instance_id_rejects_path_like_values() {
    assert_eq!(
        lock_name_for(Some("../bad")),
        "io.github.mt4110.dropsquash.lock"
    );
}
