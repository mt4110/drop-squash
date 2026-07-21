use std::sync::Mutex;

use super::{result_from, store_if_disallowed, AllowedPids};

#[test]
fn target_application_activation_is_allowed() {
    let rejected_pid = Mutex::new(None);
    store_if_disallowed(&rejected_pid, AllowedPids::new(40, 50), 40);
    assert_eq!(*rejected_pid.lock().unwrap(), None);
}

#[test]
fn recorder_application_activation_is_allowed() {
    let rejected_pid = Mutex::new(None);
    store_if_disallowed(&rejected_pid, AllowedPids::new(40, 50), 50);
    assert_eq!(*rejected_pid.lock().unwrap(), None);
}

#[test]
fn third_party_application_activation_rejects_publication() {
    let rejected_pid = Mutex::new(None);
    store_if_disallowed(&rejected_pid, AllowedPids::new(40, 50), 60);
    let error = result_from(*rejected_pid.lock().unwrap())
        .expect_err("third-party activation must fail closed")
        .to_string();

    assert!(error.contains("foreground application changed"));
    assert!(error.contains("third-party"));
}

#[test]
fn later_allowed_activation_cannot_clear_a_third_party_failure() {
    let rejected_pid = Mutex::new(None);
    let allowed = AllowedPids::new(40, 50);
    store_if_disallowed(&rejected_pid, allowed, 60);
    store_if_disallowed(&rejected_pid, allowed, 40);

    assert_eq!(*rejected_pid.lock().unwrap(), Some(60));
    assert!(result_from(*rejected_pid.lock().unwrap()).is_err());
}
