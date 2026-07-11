use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use dropsquash_core::{EncodeResult, Result};
use serde::{Deserialize, Serialize};

use crate::MetadataPolicy;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrivacyReceipt {
    pub input_name: String,
    pub output_name: String,
    pub uploaded_bytes: u64,
    pub metadata_policy: MetadataPolicy,
}

impl From<&EncodeResult> for PrivacyReceipt {
    fn from(result: &EncodeResult) -> Self {
        Self {
            input_name: file_name(&result.input_path),
            output_name: file_name(&result.output_path),
            uploaded_bytes: 0,
            metadata_policy: MetadataPolicy::Preserve,
        }
    }
}

impl PrivacyReceipt {
    pub fn save_for_result(result: &EncodeResult) -> Result<PathBuf> {
        let path = receipt_path_for(&result.output_path);
        Self::from(result).save_to_path(&path)?;
        Ok(path)
    }

    pub fn save_to_path(&self, path: &Path) -> Result<()> {
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
}

fn file_name(path: &Path) -> String {
    path.file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_string()
}

fn temporary_path_for(path: &Path) -> PathBuf {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("privacy.json");
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

pub fn receipt_path_for(output_path: &Path) -> PathBuf {
    output_path.with_extension("privacy.json")
}

#[cfg(test)]
mod tests;
