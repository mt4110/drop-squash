use tauri::Manager;

pub fn record_window_event(window: &tauri::Window, event: &tauri::WindowEvent) {
    super::qa::record_window_event(event);
    if matches!(event, tauri::WindowEvent::Destroyed) {
        super::secure_share::recording::cancel_active(&window.state());
    }
}
