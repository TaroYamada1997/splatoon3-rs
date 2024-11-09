use crate::client::SplaClient;
use crate::models::stages::{Schedule, StagesResponse};
use reqwest::StatusCode;

const BANKARA_OPEN_NEXT_ENDPOINT: &str = "https://spla3.yuu26.com/api/bankara-open/next";
const REGULAR_NOW_ENDPOINT: &str = "https://spla3.yuu26.com/api/regular/now";
const X_SCHEDULE_ENDPOINT: &str = "https://spla3.yuu26.com/api/x/schedule";

impl SplaClient {
    pub async fn get_next_bankara_open_stages(
        &self,
    ) -> Result<Vec<Schedule>, Box<dyn std::error::Error>> {
        let response = self.client.get(BANKARA_OPEN_NEXT_ENDPOINT).send().await?;

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

    pub async fn get_now_regular_open_stages(
        &self,
    ) -> Result<Vec<Schedule>, Box<dyn std::error::Error>> {
        let response = self.client.get(REGULAR_NOW_ENDPOINT).send().await?;

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

    pub async fn get_x_schedule(&self) -> Result<Vec<Schedule>, Box<dyn std::error::Error>> {
        let response = self.client.get(X_SCHEDULE_ENDPOINT).send().await?;

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
