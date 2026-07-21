use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Emitter, RunEvent, Runtime};

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("opened-files")
        .on_event(|app, event| {
            if let RunEvent::Opened { urls } = event {
                let paths = urls
                    .iter()
                    .filter_map(|url| url.to_file_path().ok())
                    .map(|path| path.display().to_string())
                    .collect::<Vec<_>>();
                if !paths.is_empty() {
                    let _ = app.emit("native-opened", paths);
                }
            }
        })
        .build()
}
