use std::path::PathBuf;

mod platform;

pub fn default_output_dir() -> PathBuf {
    if let Some(home) = std::env::var_os("HOME") {
        return PathBuf::from(home).join("Movies").join("DropSquash");
    }

    PathBuf::from("DropSquash")
}

pub fn default_config_path() -> PathBuf {
    if let Some(directory) = config_dir() {
        return directory.join("config.json");
    }

    PathBuf::from("DropSquash").join("config.json")
}

pub fn default_history_path() -> PathBuf {
    if let Some(directory) = data_dir() {
        return directory.join("history.jsonl");
    }

    PathBuf::from("DropSquash").join("history.jsonl")
}

pub fn default_license_cache_path() -> PathBuf {
    if let Some(directory) = data_dir() {
        return directory.join("license.json");
    }

    PathBuf::from("DropSquash").join("license.json")
}

fn config_dir() -> Option<PathBuf> {
    platform::config_dir()
}

fn data_dir() -> Option<PathBuf> {
    platform::data_dir()
}
