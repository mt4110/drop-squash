use async_trait::async_trait;
use dropsquash_core::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LicenseActivation {
    pub license_key_fingerprint: String,
    pub instance_id: String,
    pub valid: bool,
}

#[async_trait]
pub trait LicenseProvider: Send + Sync {
    async fn activate(&self, license_key: &str, instance_id: &str) -> Result<LicenseActivation>;
    async fn validate(&self, license_key: &str, instance_id: &str) -> Result<bool>;
    async fn deactivate(&self, license_key: &str, instance_id: &str) -> Result<()>;
}
