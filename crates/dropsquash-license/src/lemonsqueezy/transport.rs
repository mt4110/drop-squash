use std::time::Duration;

use dropsquash_core::{AppError, Result};

use super::response::LicenseApiResponse;

const API_BASE: &str = "https://api.lemonsqueezy.com/v1/licenses";
const API_BASE_ENV: &str = "DROP_SQUASH_LICENSE_API_BASE_URL";

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
            base_url: resolve_base_url(std::env::var(API_BASE_ENV).ok()),
        }
    }

    #[cfg(test)]
    pub fn test(base_url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
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
        if !response.status().is_success() {
            return Err(AppError::License(
                "License request was not accepted.".to_string(),
            ));
        }
        response.json::<LicenseApiResponse>().await.map_err(|_| {
            AppError::License("License server returned an unreadable response.".to_string())
        })
    }
}

fn resolve_base_url(value: Option<String>) -> String {
    value
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| API_BASE.to_string())
}

#[cfg(test)]
mod tests {
    use super::resolve_base_url;

    #[test]
    fn uses_override_when_present() {
        assert_eq!(
            resolve_base_url(Some(" http://127.0.0.1:9/licenses ".to_string())),
            "http://127.0.0.1:9/licenses"
        );
    }

    #[test]
    fn falls_back_for_blank_override() {
        assert_eq!(
            resolve_base_url(Some("   ".to_string())),
            "https://api.lemonsqueezy.com/v1/licenses"
        );
    }
}
