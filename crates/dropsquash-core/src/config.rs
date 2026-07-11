use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::{default_output_dir, OutputSize, Profile, Result, SourcePolicy};

pub const TRIAL_CONVERSION_LIMIT: u32 = 10;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub output_dir: PathBuf,
    pub default_profile: Profile,
    pub default_output_size: OutputSize,
    pub source_policy: SourcePolicy,
    pub write_privacy_receipt: bool,
    pub trial_conversion_limit: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            output_dir: default_output_dir(),
            default_profile: Profile::Auto,
            default_output_size: OutputSize::Auto,
            source_policy: SourcePolicy::Ask,
            write_privacy_receipt: true,
            trial_conversion_limit: TRIAL_CONVERSION_LIMIT,
        }
    }
}

impl AppConfig {
    pub fn load_or_default(path: &Path) -> Result<Self> {
        match std::fs::read(path) {
            Ok(bytes) => Ok(serde_json::from_slice(&bytes)?),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Self::default()),
            Err(error) => Err(error.into()),
        }
    }

    pub fn save_to_path(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("config.json");
        let unique_suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or_default();
        let temporary_path = path.with_file_name(format!(
            ".{file_name}.{}.{}.tmp",
            std::process::id(),
            unique_suffix
        ));
        std::fs::write(&temporary_path, serde_json::to_vec_pretty(self)?)?;

        #[cfg(target_os = "windows")]
        if path.exists() {
            std::fs::remove_file(path)?;
        }

        match std::fs::rename(&temporary_path, path) {
            Ok(()) => {}
            Err(error) => {
                let _ = std::fs::remove_file(&temporary_path);
                return Err(error.into());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
