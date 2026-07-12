mod activation;
mod redaction;
mod response;
mod transport;

use async_trait::async_trait;
use dropsquash_core::Result;

use crate::{LicenseActivation, LicenseProvider};
use transport::LicenseApiClient;

#[derive(Debug, Clone)]
pub struct LemonSqueezyProvider {
    client: LicenseApiClient,
}

impl Default for LemonSqueezyProvider {
    fn default() -> Self {
        Self {
            client: LicenseApiClient::production(),
        }
    }
}

#[async_trait]
impl LicenseProvider for LemonSqueezyProvider {
    async fn activate(&self, license_key: &str, instance_id: &str) -> Result<LicenseActivation> {
        let response = self
            .client
            .post(
                "activate",
                &[("license_key", license_key), ("instance_name", instance_id)],
            )
            .await?;
        activation::from_response(license_key, response)
    }

    async fn validate(&self, license_key: &str, instance_id: &str) -> Result<bool> {
        let response = self
            .client
            .post(
                "validate",
                &[("license_key", license_key), ("instance_id", instance_id)],
            )
            .await?;
        Ok(response.valid.unwrap_or(false))
    }

    async fn deactivate(&self, license_key: &str, instance_id: &str) -> Result<()> {
        let response = self
            .client
            .post(
                "deactivate",
                &[("license_key", license_key), ("instance_id", instance_id)],
            )
            .await?;
        activation::deactivate_from_response(license_key, response)
    }
}

#[cfg(test)]
mod tests;
