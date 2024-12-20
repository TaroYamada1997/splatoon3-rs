use reqwest::StatusCode;

use crate::{
    client::SplaClient,
    models::stages::{Schedule, StagesResponse},
};

const BANKARA_OPEN_NEXT_ENDPOINT: &str = "bankara-open/next";
const BANKARA_CHALLENGE_SCHEDULE_ENDPOINT: &str = "bankara-challenge/schedule";

impl SplaClient {
    pub async fn get_next_bankara_open_stages(
        &self,
    ) -> Result<Vec<Schedule>, Box<dyn std::error::Error>> {
        let base_url = &self.base_url;

        let response = self
            .client
            .get(format!("{base_url}/{BANKARA_OPEN_NEXT_ENDPOINT}"))
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => {
                let bankara_open_stages: StagesResponse = response.json().await?;
                Ok(bankara_open_stages.results)
            }
            status_code => {
                Err(format!("API request failed with status code: {}", status_code).into())
            }
        }
    }

    pub async fn get_bankara_challenge_schedule(
        &self,
    ) -> Result<Vec<Schedule>, Box<dyn std::error::Error>> {
        let base_url = &self.base_url;

        let response = self
            .client
            .get(format!("{base_url}/{BANKARA_CHALLENGE_SCHEDULE_ENDPOINT}"))
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => {
                let bankara_challenge_stages: StagesResponse = response.json().await?;
                Ok(bankara_challenge_stages.results)
            }
            status_code => {
                Err(format!("API request failed with status code: {}", status_code).into())
            }
        }
    }
}
