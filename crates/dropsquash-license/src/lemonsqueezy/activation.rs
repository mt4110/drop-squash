use dropsquash_core::{AppError, Result};

use crate::{license_key_fingerprint, LicenseActivation};

use super::redaction::redact_license_key;
use super::response::LicenseApiResponse;

pub(super) fn from_response(
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

pub(super) fn deactivate_from_response(
    license_key: &str,
    response: LicenseApiResponse,
) -> Result<()> {
    if response.deactivated.unwrap_or(false) {
        return Ok(());
    }
    Err(license_error(license_key, response))
}

fn license_error(license_key: &str, response: LicenseApiResponse) -> AppError {
    AppError::License(redact_license_key(&response.friendly_error(), license_key))
}
