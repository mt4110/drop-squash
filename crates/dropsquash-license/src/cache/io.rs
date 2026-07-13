use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use dropsquash_core::AppError;

use crate::LicenseCache;

impl LicenseCache {
    pub fn load_or_default(path: &Path) -> dropsquash_core::Result<Self> {
        match std::fs::read(path) {
            Ok(bytes) => load_cache_bytes(&bytes),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Self::default()),
            Err(error) => Err(error.into()),
        }
    }

    pub fn save_to_path(&self, path: &Path) -> dropsquash_core::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let temporary_path = temporary_path_for(path);
        std::fs::write(&temporary_path, serde_json::to_vec_pretty(self)?)?;

        #[cfg(target_os = "windows")]
        if path.exists() {
            std::fs::remove_file(path)?;
        }

        match std::fs::rename(&temporary_path, path) {
            Ok(()) => Ok(()),
            Err(error) => {
                let _ = std::fs::remove_file(&temporary_path);
                Err(error.into())
            }
        }
    }

    pub fn forget_path(path: &Path) -> dropsquash_core::Result<()> {
        match std::fs::remove_file(path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error.into()),
        }
    }
}

fn load_cache_bytes(bytes: &[u8]) -> dropsquash_core::Result<LicenseCache> {
    let value = serde_json::from_slice(bytes)?;
    reject_raw_key_fields(&value)?;
    Ok(serde_json::from_value(value)?)
}

fn reject_raw_key_fields(value: &serde_json::Value) -> dropsquash_core::Result<()> {
    match value {
        serde_json::Value::Object(object) => {
            if object.keys().any(|key| is_raw_key_field(key)) {
                return Err(AppError::License(
                    "license cache contains a raw license key field".to_string(),
                ));
            }
            for child in object.values() {
                reject_raw_key_fields(child)?;
            }
        }
        serde_json::Value::Array(values) => {
            for child in values {
                reject_raw_key_fields(child)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn is_raw_key_field(key: &str) -> bool {
    let normalized = key
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect::<String>();
    matches!(
        normalized.as_str(),
        "licensekey" | "rawkey" | "rawlicensekey"
    )
}

fn temporary_path_for(path: &Path) -> PathBuf {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("license.json");
    let unique_suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    path.with_file_name(format!(
        ".{file_name}.{}.{}.tmp",
        std::process::id(),
        unique_suffix
    ))
}
