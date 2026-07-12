use super::is_older_than_head;

#[test]
fn reports_artifact_older_than_head() {
    assert!(is_older_than_head(10, 11));
}

#[test]
fn accepts_artifact_newer_than_head() {
    assert!(!is_older_than_head(12, 11));
}

#[test]
fn accepts_artifact_with_same_epoch_as_head() {
    assert!(!is_older_than_head(11, 11));
}
