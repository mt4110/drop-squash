use dropsquash_core::{AppError, CaptureRect, Result};

use super::{SckDisplayCandidate, SckShareableContentSnapshot, SckWindowCandidate};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SckCaptureTargetKind {
    Window,
    Display,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SckCaptureTarget {
    pub kind: SckCaptureTargetKind,
    pub id: u32,
    pub frame: CaptureRect,
    pub owner_pid: Option<i32>,
}

pub fn select_strict_reveal_window_target(
    snapshot: &SckShareableContentSnapshot,
) -> Result<SckCaptureTarget> {
    let eligible = snapshot
        .windows
        .iter()
        .filter(is_strict_reveal_window_candidate)
        .collect::<Vec<_>>();
    match eligible.as_slice() {
        [window] => Ok(SckCaptureTarget {
            kind: SckCaptureTargetKind::Window,
            id: window.window_id,
            frame: window.frame,
            owner_pid: window.owner_pid,
        }),
        [] => Err(selection_error("no eligible active window candidate")),
        _ => Err(selection_error(
            "multiple eligible active window candidates",
        )),
    }
}

pub fn select_explicit_display_target(
    snapshot: &SckShareableContentSnapshot,
    display_id: u32,
) -> Result<SckCaptureTarget> {
    let display = snapshot
        .displays
        .iter()
        .find(|candidate| candidate.display_id == display_id)
        .ok_or_else(|| selection_error("requested display candidate was not found"))?;
    Ok(display_target(display))
}

pub fn select_explicit_window_target(
    snapshot: &SckShareableContentSnapshot,
    window_id: u32,
) -> Result<SckCaptureTarget> {
    let window = snapshot
        .windows
        .iter()
        .find(|candidate| candidate.window_id == window_id)
        .ok_or_else(|| selection_error("requested window candidate was not found"))?;
    Ok(window_target(window))
}

fn is_strict_reveal_window_candidate(window: &&SckWindowCandidate) -> bool {
    window.on_screen
        && window.active
        && window.has_owner
        && window.layer == 0
        && window.frame.width > 0
        && window.frame.height > 0
}

fn display_target(display: &SckDisplayCandidate) -> SckCaptureTarget {
    SckCaptureTarget {
        kind: SckCaptureTargetKind::Display,
        id: display.display_id,
        frame: display.frame,
        owner_pid: None,
    }
}

fn window_target(window: &SckWindowCandidate) -> SckCaptureTarget {
    SckCaptureTarget {
        kind: SckCaptureTargetKind::Window,
        id: window.window_id,
        frame: window.frame,
        owner_pid: window.owner_pid,
    }
}

fn selection_error(reason: &str) -> AppError {
    AppError::InvalidConfig(format!(
        "Secure Share ScreenCaptureKit target selection failed closed: {reason}"
    ))
}
