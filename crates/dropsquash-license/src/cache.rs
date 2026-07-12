mod io;
mod pro;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

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

pub fn license_key_fingerprint(license_key: &str) -> String {
    let digest = Sha256::digest(license_key.trim().as_bytes());
    digest
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>()
}

#[cfg(test)]
mod tests;
