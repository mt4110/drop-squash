use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LicenseCache {
    pub instance_id: Option<String>,
    pub license_key_fingerprint: Option<String>,
    pub valid: bool,
}
