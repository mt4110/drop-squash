use std::path::PathBuf;

#[cfg(target_os = "macos")]
pub(super) fn config_dir() -> Option<PathBuf> {
    macos_support_dir()
}

#[cfg(target_os = "macos")]
pub(super) fn data_dir() -> Option<PathBuf> {
    macos_support_dir()
}

#[cfg(target_os = "windows")]
pub(super) fn config_dir() -> Option<PathBuf> {
    app_data_dir()
}

#[cfg(target_os = "windows")]
pub(super) fn data_dir() -> Option<PathBuf> {
    app_data_dir()
}

#[cfg(target_os = "linux")]
pub(super) fn config_dir() -> Option<PathBuf> {
    if let Some(config_home) = std::env::var_os("XDG_CONFIG_HOME") {
        return Some(PathBuf::from(config_home).join("DropSquash"));
    }
    std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config").join("DropSquash"))
}

#[cfg(target_os = "linux")]
pub(super) fn data_dir() -> Option<PathBuf> {
    if let Some(data_home) = std::env::var_os("XDG_DATA_HOME") {
        return Some(PathBuf::from(data_home).join("DropSquash"));
    }
    std::env::var_os("HOME").map(|home| {
        PathBuf::from(home)
            .join(".local")
            .join("share")
            .join("DropSquash")
    })
}

#[cfg(target_os = "macos")]
fn macos_support_dir() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|home| {
        PathBuf::from(home)
            .join("Library")
            .join("Application Support")
            .join("DropSquash")
    })
}

#[cfg(target_os = "windows")]
fn app_data_dir() -> Option<PathBuf> {
    std::env::var_os("APPDATA").map(|app_data| PathBuf::from(app_data).join("DropSquash"))
}

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
pub(super) fn config_dir() -> Option<PathBuf> {
    None
}

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
pub(super) fn data_dir() -> Option<PathBuf> {
    None
}
