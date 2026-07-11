use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct LicenseApiResponse {
    pub activated: Option<bool>,
    pub valid: Option<bool>,
    pub deactivated: Option<bool>,
    pub error: Option<String>,
    pub instance: Option<LicenseInstance>,
}

impl LicenseApiResponse {
    pub fn friendly_error(&self) -> String {
        self.error
            .clone()
            .unwrap_or_else(|| "License request was not accepted.".to_string())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct LicenseInstance {
    pub id: String,
}
