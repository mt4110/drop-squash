use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::Path;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct LicenseCache {
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
        std::fs::write(path, serde_json::to_vec_pretty(self)?)?;
        Ok(())
    }

    pub fn permits_pro(&self, now_unix: u64) -> bool {
        self.valid
            && self
                .offline_grace_until_unix
                .is_some_and(|until| now_unix <= until)
    }
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
