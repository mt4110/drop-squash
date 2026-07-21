#[cfg(unix)]
mod unix;

pub struct Guard {
    #[cfg(unix)]
    file: std::fs::File,
}

pub fn prepare() -> Result<Option<Guard>, String> {
    #[cfg(unix)]
    {
        unix::prepare()
    }
    #[cfg(not(unix))]
    {
        Ok(Some(Guard {}))
    }
}

impl Guard {
    pub fn attach<R: tauri::Runtime>(self, app: &tauri::AppHandle<R>) {
        #[cfg(unix)]
        unix::attach(self.file, app.clone());
    }
}

pub fn focus_existing<R: tauri::Runtime>(app: &tauri::AppHandle<R>) {
    #[cfg(unix)]
    unix::focus_main_window(app);
}

pub fn qa_parallel_instance_requested() -> bool {
    #[cfg(unix)]
    {
        unix::qa_parallel_instance_requested()
    }
    #[cfg(not(unix))]
    {
        false
    }
}
