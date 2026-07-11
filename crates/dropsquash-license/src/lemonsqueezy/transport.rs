use std::time::Duration;

use dropsquash_core::{AppError, Result};

use super::response::LicenseApiResponse;

const API_BASE: &str = "https://api.lemonsqueezy.com/v1/licenses";

#[derive(Debug, Clone)]
pub struct LicenseApiClient {
    client: reqwest::Client,
    base_url: String,
}

impl LicenseApiClient {
    pub fn production() -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("license HTTP client should be constructible");
        Self {
            client,
            base_url: API_BASE.to_string(),
        }
    }

    pub async fn post(&self, endpoint: &str, form: &[(&str, &str)]) -> Result<LicenseApiResponse> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self
            .client
            .post(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .form(form)
            .send()
            .await
            .map_err(|_| AppError::License("License server is unreachable.".to_string()))?;
        response.json::<LicenseApiResponse>().await.map_err(|_| {
            AppError::License("License server returned an unreadable response.".to_string())
        })
    }
}
