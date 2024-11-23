use reqwest::StatusCode;

use crate::{
    client::SplaClient,
    models::stages::{Schedule, StagesResponse},
};

const REGULAR_NOW_ENDPOINT: &str = "regular/now";

impl SplaClient {
    pub async fn get_now_regular_open_stages(
        &self,
    ) -> Result<Vec<Schedule>, Box<dyn std::error::Error>> {
        let base_url = &self.base_url;

        let response = self
            .client
            .get(format!("{base_url}/{REGULAR_NOW_ENDPOINT}"))
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => {
                let regular_stages: StagesResponse = response.json().await?;
                Ok(regular_stages.results)
            }
            status_code => {
                Err(format!("API request failed with status code: {}", status_code).into())
            }
        }
    }
}
