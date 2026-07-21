use dropsquash_core::{CaptureRect, Result};

use super::{select_explicit_window_target, SckCaptureTarget, SckShareableContentSnapshot};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SckWindowSelection {
    pub window_id: u32,
    pub owner_pid: i32,
    pub title: Option<String>,
    pub frame: CaptureRect,
}

pub fn select_attested_window_target(
    snapshot: &SckShareableContentSnapshot,
    selection: &SckWindowSelection,
) -> Result<SckCaptureTarget> {
    let target = select_explicit_window_target(snapshot, selection.window_id)?;
    let window = snapshot
        .windows
        .iter()
        .find(|window| window.window_id == selection.window_id);
    let title = window.and_then(|window| window.title.as_ref());
    if target.owner_pid != Some(selection.owner_pid)
        || target.frame != selection.frame
        || title != selection.title.as_ref()
    {
        return Err(dropsquash_core::AppError::InvalidConfig(
            "Secure Share ScreenCaptureKit target selection failed closed: selected window changed before capture"
                .to_string(),
        ));
    }
    Ok(target)
}
