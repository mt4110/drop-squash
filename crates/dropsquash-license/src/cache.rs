use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct LicenseCache {
    pub instance_name: Option<String>,
    pub instance_id: Option<String>,
    pub license_key_fingerprint: Option<String>,
    pub activation_id: Option<String>,
    pub validated_at_unix: Option<u64>,
    pub offline_grace_until_unix: Option<u64>,
    pub valid: bool,
}

impl LicenseCache {
    pub fn load_or_default(path: &Path) -> dropsquash_core::Result<Self> {
        match std::fs::read(path) {
            Ok(bytes) => Ok(serde_json::from_slice(&bytes)?),
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
            Ok(()) => {}
            Err(error) => {
                let _ = std::fs::remove_file(&temporary_path);
                return Err(error.into());
            }
        }
        Ok(())
    }

    pub fn permits_pro(&self, now_unix: u64) -> bool {
        self.valid
            && self.has_activation_identity()
            && self
                .offline_grace_until_unix
                .is_some_and(|until| now_unix <= until)
    }

    fn has_activation_identity(&self) -> bool {
        self.instance_id
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty())
            && self
                .license_key_fingerprint
                .as_deref()
                .is_some_and(|value| !value.trim().is_empty())
    }
}

fn temporary_path_for(path: &Path) -> std::path::PathBuf {
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

pub fn license_key_fingerprint(license_key: &str) -> String {
    let digest = Sha256::digest(license_key.trim().as_bytes());
    digest
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>()
}

#[cfg(test)]
mod tests;
