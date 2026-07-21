use objc2_app_kit::{NSWorkspace, NSWorkspaceScreensDidSleepNotification};

use super::{result_from, LifecycleWatch};

#[test]
fn lifecycle_event_rejects_publication() {
    let error = result_from(Some("macOS screens slept"))
        .expect_err("lifecycle event must fail closed")
        .to_string();

    assert!(error.contains("lifecycle changed"));
    assert!(error.contains("screens slept"));
}

#[test]
fn no_lifecycle_event_allows_publication_boundary_to_continue() {
    assert!(result_from(None).is_ok());
}

#[test]
fn posted_workspace_lifecycle_notification_rejects_publication() {
    let watch = LifecycleWatch::start();
    let center = NSWorkspace::sharedWorkspace().notificationCenter();
    unsafe {
        center.postNotificationName_object(NSWorkspaceScreensDidSleepNotification, None);
    }

    let error = watch
        .finish()
        .expect_err("workspace lifecycle notification must fail closed")
        .to_string();
    assert!(error.contains("screens slept"));
}
