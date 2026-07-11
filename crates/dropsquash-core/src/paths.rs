use std::path::PathBuf;

pub fn default_output_dir() -> PathBuf {
    if let Some(home) = std::env::var_os("HOME") {
        return PathBuf::from(home).join("Movies").join("DropSquash");
    }

    PathBuf::from("DropSquash")
}

pub fn default_config_path() -> PathBuf {
    #[cfg(target_os = "macos")]
    if let Some(home) = std::env::var_os("HOME") {
        return PathBuf::from(home)
            .join("Library")
            .join("Application Support")
            .join("DropSquash")
            .join("config.json");
    }

    #[cfg(target_os = "windows")]
    if let Some(app_data) = std::env::var_os("APPDATA") {
        return PathBuf::from(app_data)
            .join("DropSquash")
            .join("config.json");
    }

    #[cfg(target_os = "linux")]
    if let Some(config_home) = std::env::var_os("XDG_CONFIG_HOME") {
        return PathBuf::from(config_home)
            .join("DropSquash")
            .join("config.json");
    }

    #[cfg(target_os = "linux")]
    if let Some(home) = std::env::var_os("HOME") {
        return PathBuf::from(home)
            .join(".config")
            .join("DropSquash")
            .join("config.json");
    }

    PathBuf::from("DropSquash").join("config.json")
}

pub fn default_history_path() -> PathBuf {
    #[cfg(target_os = "macos")]
    if let Some(home) = std::env::var_os("HOME") {
        return PathBuf::from(home)
            .join("Library")
            .join("Application Support")
            .join("DropSquash")
            .join("history.jsonl");
    }

    #[cfg(target_os = "windows")]
    if let Some(app_data) = std::env::var_os("APPDATA") {
        return PathBuf::from(app_data)
            .join("DropSquash")
            .join("history.jsonl");
    }

    #[cfg(target_os = "linux")]
    if let Some(data_home) = std::env::var_os("XDG_DATA_HOME") {
        return PathBuf::from(data_home)
            .join("DropSquash")
            .join("history.jsonl");
    }

    #[cfg(target_os = "linux")]
    if let Some(home) = std::env::var_os("HOME") {
        return PathBuf::from(home)
            .join(".local")
            .join("share")
            .join("DropSquash")
            .join("history.jsonl");
    }

    PathBuf::from("DropSquash").join("history.jsonl")
}

pub fn default_license_cache_path() -> PathBuf {
    #[cfg(target_os = "macos")]
    if let Some(home) = std::env::var_os("HOME") {
        return PathBuf::from(home)
            .join("Library")
            .join("Application Support")
            .join("DropSquash")
            .join("license.json");
    }

    #[cfg(target_os = "windows")]
    if let Some(app_data) = std::env::var_os("APPDATA") {
        return PathBuf::from(app_data)
            .join("DropSquash")
            .join("license.json");
    }

    #[cfg(target_os = "linux")]
    if let Some(data_home) = std::env::var_os("XDG_DATA_HOME") {
        return PathBuf::from(data_home)
            .join("DropSquash")
            .join("license.json");
    }

    PathBuf::from("DropSquash").join("license.json")
}
