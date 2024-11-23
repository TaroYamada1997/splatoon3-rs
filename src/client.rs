use reqwest::{header, Client};
use std::time::Duration;

const DEFAULT_TIMEOUT: u64 = 30;

pub struct SplaClient {
    pub(crate) client: Client,
    pub base_url: String,
}

impl SplaClient {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let base_url = std::env::var("DEFAULT_BASE_URL")
            .unwrap_or_else(|_| "https://spla3.yuu26.com/api".to_string());
        Self::new_with_base_url(base_url)
    }

    pub fn new_with_base_url(base_url: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static("SplaClient/0.1"),
        );

        let client = Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT))
            .default_headers(headers)
            .build()?;

        Ok(Self {
            client,
            base_url: base_url.to_string(),
        })
    }

    pub(crate) fn _build_url(&self, endpoint: &str) -> String {
        format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            endpoint.trim_start_matches('/')
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_client() {
        let client = SplaClient::new();
        assert!(client.is_ok());
    }

    #[test]
    fn test_new_client_with_custom_base_url() {
        let base_url = "https://custom.api.com".to_string();
        let client = SplaClient::new_with_base_url(base_url.clone());
        assert!(client.is_ok());
        assert_eq!(client.unwrap().base_url, base_url.clone());
    }
}
