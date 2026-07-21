use std::time::Duration;

use dropsquash_core::Result;

use super::super::{
    request_shareable_content_snapshot, select_attested_window_target, SckShareableContentRequest,
    SckWindowSelection,
};

pub(super) fn after_stop(selection: &SckWindowSelection, timeout: Duration) -> Result<()> {
    let snapshot = request_shareable_content_snapshot(SckShareableContentRequest {
        exclude_desktop_windows: true,
        on_screen_windows_only: true,
        timeout,
    })?;
    verify_snapshot(&snapshot, selection)
}

pub(crate) fn verify_snapshot(
    snapshot: &super::super::SckShareableContentSnapshot,
    selection: &SckWindowSelection,
) -> Result<()> {
    select_attested_window_target(snapshot, selection).map(|_| ())
}
