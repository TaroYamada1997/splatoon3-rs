use reqwest::StatusCode;

use crate::{
    client::SplaClient,
    models::stages::{Schedule, StagesResponse},
};

const X_SCHEDULE_ENDPOINT: &str = "x/schedule";

impl SplaClient {
    pub async fn get_x_schedule(&self) -> Result<Vec<Schedule>, Box<dyn std::error::Error>> {
        let base_url = &self.base_url;

        let response = self
            .client
            .get(format!("{base_url}/{X_SCHEDULE_ENDPOINT}"))
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => {
                let x_schedule: StagesResponse = response.json().await?;
                Ok(x_schedule.results)
            }
            status_code => {
                Err(format!("API request failed with status code: {}", status_code).into())
            }
        }
    }
}
