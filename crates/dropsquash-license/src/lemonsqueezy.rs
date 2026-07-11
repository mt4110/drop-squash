use async_trait::async_trait;
use dropsquash_core::{AppError, Result};

use crate::{LicenseActivation, LicenseProvider};

#[derive(Debug, Clone, Default)]
pub struct LemonSqueezyProvider;

#[async_trait]
impl LicenseProvider for LemonSqueezyProvider {
    async fn activate(&self, _license_key: &str, _instance_id: &str) -> Result<LicenseActivation> {
        Err(AppError::License(
            "Lemon Squeezy activation is intentionally not implemented in Phase 0".to_string(),
        ))
    }

    async fn validate(&self, _activation: &LicenseActivation) -> Result<bool> {
        Err(AppError::License(
            "Lemon Squeezy validation is intentionally not implemented in Phase 0".to_string(),
        ))
    }

    async fn deactivate(&self, _activation: &LicenseActivation) -> Result<()> {
        Err(AppError::License(
            "Lemon Squeezy deactivation is intentionally not implemented in Phase 0".to_string(),
        ))
    }
}
