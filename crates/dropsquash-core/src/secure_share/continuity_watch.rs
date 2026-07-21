pub const WATCH_DISPLAY_CONFIGURATION: &str = "display_configuration";
pub const WATCH_CORE_GRAPHICS_WINDOW: &str = "core_graphics_window";
pub const WATCH_ACCESSIBILITY_GEOMETRY: &str = "accessibility_geometry";
pub const WATCH_ACCESSIBILITY_WINDOW_EVENTS: &str = "accessibility_window_events";
pub const WATCH_MACOS_LIFECYCLE_NOTIFICATIONS: &str = "macos_lifecycle_notifications";
pub const WATCH_FOREGROUND_PID_ALLOWLIST: &str = "foreground_activation_pid_allowlist";

pub fn required_capture_continuity_watches() -> [&'static str; 6] {
    [
        WATCH_DISPLAY_CONFIGURATION,
        WATCH_CORE_GRAPHICS_WINDOW,
        WATCH_ACCESSIBILITY_GEOMETRY,
        WATCH_ACCESSIBILITY_WINDOW_EVENTS,
        WATCH_MACOS_LIFECYCLE_NOTIFICATIONS,
        WATCH_FOREGROUND_PID_ALLOWLIST,
    ]
}
