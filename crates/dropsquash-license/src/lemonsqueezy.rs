mod redaction;
mod response;
mod transport;

use async_trait::async_trait;
use dropsquash_core::{AppError, Result};

use crate::{license_key_fingerprint, LicenseActivation, LicenseProvider};
use redaction::redact_license_key;
use response::LicenseApiResponse;
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
        activation_from_response(license_key, response)
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
        deactivation_from_response(license_key, response)
    }
}

fn activation_from_response(
    license_key: &str,
    response: LicenseApiResponse,
) -> Result<LicenseActivation> {
    let valid = response.activated.unwrap_or(false);
    if !valid {
        return Err(license_error(license_key, response));
    }
    let instance_id = response
        .instance
        .map(|instance| instance.id)
        .ok_or_else(|| {
            AppError::License("License activation did not return an instance id.".to_string())
        })?;
    if instance_id.trim().is_empty() {
        return Err(AppError::License(
            "License activation returned an empty instance id.".to_string(),
        ));
    }
    Ok(LicenseActivation {
        license_key_fingerprint: license_key_fingerprint(license_key),
        instance_id,
        valid,
    })
}

fn license_error(license_key: &str, response: LicenseApiResponse) -> AppError {
    AppError::License(redact_license_key(&response.friendly_error(), license_key))
}

fn deactivation_from_response(license_key: &str, response: LicenseApiResponse) -> Result<()> {
    if response.deactivated.unwrap_or(false) {
        return Ok(());
    }
    Err(license_error(license_key, response))
}

#[cfg(test)]
mod tests;
