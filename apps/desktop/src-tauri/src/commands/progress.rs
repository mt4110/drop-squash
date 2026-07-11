use dropsquash_encoder::EncodeProgressReporter;
use tauri::Emitter;

#[derive(Clone)]
pub struct WindowProgressReporter {
    pub window: tauri::WebviewWindow,
}

impl EncodeProgressReporter for WindowProgressReporter {
    fn report(&self, fraction: f32) {
        let percent = (fraction.clamp(0.0, 1.0) * 100.0).round() as u8;
        let _ = self.window.emit("conversion-progress", percent);
    }
}
