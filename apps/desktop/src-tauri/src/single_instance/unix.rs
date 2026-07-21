use std::fs::{File, OpenOptions};
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};

use tauri::Manager;

#[cfg(target_os = "macos")]
use objc2_app_kit::{NSApplicationActivationOptions, NSRunningApplication};
#[cfg(target_os = "macos")]
use objc2_foundation::ns_string;

use super::Guard;

const LOCK_NAME: &str = "io.github.mt4110.dropsquash.lock";
const QA_INSTANCE_ID: &str = "DROP_SQUASH_QA_INSTANCE_ID";
struct HeldLock {
    _file: File,
}

pub fn prepare() -> Result<Option<Guard>, String> {
    prepare_at(&lock_path())
}

fn prepare_at(path: &Path) -> Result<Option<Guard>, String> {
    let file =
        open_lock(path).map_err(|error| format!("failed to open single-instance file: {error}"))?;
    match try_lock(&file) {
        Ok(()) => Ok(Some(Guard { file })),
        Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
            let _ = activate_existing();
            Ok(None)
        }
        Err(error) => Err(format!("failed to lock single-instance file: {error}")),
    }
}

pub fn attach<R: tauri::Runtime>(file: File, app: tauri::AppHandle<R>) {
    app.manage(HeldLock { _file: file });
}

fn open_lock(path: &Path) -> std::io::Result<File> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(false)
        .open(path)
}

fn try_lock(file: &File) -> std::io::Result<()> {
    let result = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
    if result == 0 {
        return Ok(());
    }
    Err(std::io::Error::last_os_error())
}

#[cfg(target_os = "macos")]
fn activate_existing() -> std::io::Result<()> {
    let running = NSRunningApplication::runningApplicationsWithBundleIdentifier(ns_string!(
        "io.github.mt4110.dropsquash"
    ));
    let Some(app) = running.firstObject() else {
        return Ok(());
    };
    app.activateWithOptions(NSApplicationActivationOptions::ActivateAllWindows)
        .then_some(())
        .ok_or_else(std::io::Error::last_os_error)
}

#[cfg(not(target_os = "macos"))]
fn activate_existing() -> std::io::Result<()> {
    Ok(())
}

fn lock_path() -> PathBuf {
    std::env::temp_dir().join(lock_name())
}

fn lock_name() -> String {
    lock_name_for(std::env::var(QA_INSTANCE_ID).ok().as_deref())
}

pub fn qa_parallel_instance_requested() -> bool {
    std::env::var(QA_INSTANCE_ID)
        .ok()
        .is_some_and(|id| lock_name_for(Some(&id)) != LOCK_NAME)
}

fn lock_name_for(id: Option<&str>) -> String {
    let Some(id) = id.filter(|value| {
        !value.is_empty()
            && value
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
    }) else {
        return LOCK_NAME.to_string();
    };
    format!("io.github.mt4110.dropsquash.{id}.lock")
}

pub fn focus_main_window<R: tauri::Runtime>(app: &tauri::AppHandle<R>) {
    let _ = activate_existing();
    let _ = app.show();
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

#[cfg(test)]
mod tests;
