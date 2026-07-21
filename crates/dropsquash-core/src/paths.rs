use std::path::PathBuf;

mod platform;

pub fn default_output_dir() -> PathBuf {
    if let Some(home) = env_home() {
        return PathBuf::from(home).join("Movies").join("DropSquash");
    }
    PathBuf::from("DropSquash")
}

pub fn default_config_path() -> PathBuf {
    state_file("config.json", config_dir)
}

pub fn default_history_path() -> PathBuf {
    state_file("history.jsonl", data_dir)
}

pub fn default_license_cache_path() -> PathBuf {
    state_file("license.json", data_dir)
}

pub fn default_evidence_signing_key_path() -> PathBuf {
    default_config_path().with_file_name("secure-share-ed25519.pk8")
}

fn config_dir() -> Option<PathBuf> {
    platform::config_dir()
}

fn data_dir() -> Option<PathBuf> {
    platform::data_dir()
}

fn state_file(name: &str, fallback: fn() -> Option<PathBuf>) -> PathBuf {
    state_dir()
        .or_else(fallback)
        .map(|dir| dir.join(name))
        .unwrap_or_else(|| PathBuf::from("DropSquash").join(name))
}

fn state_dir() -> Option<PathBuf> {
    std::env::var_os("DROP_SQUASH_APP_STATE_DIR").map(PathBuf::from)
}

fn env_home() -> Option<std::ffi::OsString> {
    std::env::var_os("DROP_SQUASH_HOME").or_else(|| std::env::var_os("HOME"))
}

#[cfg(test)]
mod tests;
