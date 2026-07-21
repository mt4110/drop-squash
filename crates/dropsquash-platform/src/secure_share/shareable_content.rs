use dropsquash_core::{AppError, CaptureRect, Result};
use objc2_core_foundation::CGRect;
use objc2_screen_capture_kit::{SCDisplay, SCShareableContent, SCWindow};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SckWindowCandidate {
    pub window_id: u32,
    pub title: Option<String>,
    pub frame: CaptureRect,
    pub layer: isize,
    pub on_screen: bool,
    pub active: bool,
    pub has_title: bool,
    pub has_owner: bool,
    pub owner_name: Option<String>,
    pub owner_pid: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SckDisplayCandidate {
    pub display_id: u32,
    pub width_points: u32,
    pub height_points: u32,
    pub frame: CaptureRect,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SckShareableContentSnapshot {
    pub windows: Vec<SckWindowCandidate>,
    pub displays: Vec<SckDisplayCandidate>,
}

pub fn snapshot_shareable_content(
    content: &SCShareableContent,
) -> Result<SckShareableContentSnapshot> {
    let windows = unsafe { content.windows() }
        .to_vec()
        .iter()
        .map(|window| snapshot_window(window))
        .collect::<Result<Vec<_>>>()?;
    let displays = unsafe { content.displays() }
        .to_vec()
        .iter()
        .map(|display| snapshot_display(display))
        .collect::<Result<Vec<_>>>()?;
    if windows.is_empty() && displays.is_empty() {
        return Err(AppError::InvalidConfig(
            "Secure Share ScreenCaptureKit shareable content is empty".to_string(),
        ));
    }
    Ok(SckShareableContentSnapshot { windows, displays })
}

fn snapshot_window(window: &SCWindow) -> Result<SckWindowCandidate> {
    let title = unsafe { window.title() }.map(|title| title.to_string());
    let owner = unsafe { window.owningApplication() };
    let owner_name = owner
        .as_ref()
        .map(|app| unsafe { app.applicationName() }.to_string());
    Ok(SckWindowCandidate {
        window_id: unsafe { window.windowID() },
        has_title: title.is_some(),
        title,
        frame: capture_rect_from_cg_rect(unsafe { window.frame() })?,
        layer: unsafe { window.windowLayer() },
        on_screen: unsafe { window.isOnScreen() },
        active: unsafe { window.isActive() },
        has_owner: owner.is_some(),
        owner_name,
        owner_pid: owner.map(|app| unsafe { app.processID() }),
    })
}

fn snapshot_display(display: &SCDisplay) -> Result<SckDisplayCandidate> {
    Ok(SckDisplayCandidate {
        display_id: unsafe { display.displayID() },
        width_points: positive_u32(unsafe { display.width() }, "display width")?,
        height_points: positive_u32(unsafe { display.height() }, "display height")?,
        frame: capture_rect_from_cg_rect(unsafe { display.frame() })?,
    })
}

fn capture_rect_from_cg_rect(rect: CGRect) -> Result<CaptureRect> {
    Ok(CaptureRect {
        x: i32_from_float(rect.origin.x, "rect x")?,
        y: i32_from_float(rect.origin.y, "rect y")?,
        width: positive_u32(rect.size.width, "rect width")?,
        height: positive_u32(rect.size.height, "rect height")?,
    })
}

fn i32_from_float(value: f64, name: &str) -> Result<i32> {
    if !value.is_finite() || value < i32::MIN as f64 || value > i32::MAX as f64 {
        return Err(invalid_number(name));
    }
    Ok(value.round() as i32)
}

fn positive_u32(value: impl ToF64, name: &str) -> Result<u32> {
    let value = value.to_f64();
    if !value.is_finite() || value <= 0.0 || value > u32::MAX as f64 {
        return Err(invalid_number(name));
    }
    Ok(value.round() as u32)
}

trait ToF64 {
    fn to_f64(self) -> f64;
}

impl ToF64 for f64 {
    fn to_f64(self) -> f64 {
        self
    }
}

impl ToF64 for isize {
    fn to_f64(self) -> f64 {
        self as f64
    }
}

fn invalid_number(name: &str) -> AppError {
    AppError::InvalidConfig(format!(
        "Secure Share ScreenCaptureKit shareable content {name} is invalid"
    ))
}
