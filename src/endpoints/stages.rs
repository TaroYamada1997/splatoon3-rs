use crate::client::SplaClient;
use crate::models::stages::{BankaraOpenStagesResponse, Schedule};
use reqwest::StatusCode;

const BANKARA_OPEN_NEXT_ENDPOINT: &str = "https://spla3.yuu26.com/api/bankara-open/next";

impl SplaClient {
    pub async fn get_next_bankara_open_stages(&self) -> Result<Vec<Schedule>, Box<dyn std::error::Error>> {
        let response = self.client
            .get(BANKARA_OPEN_NEXT_ENDPOINT)
            .send()
            .await?;

        println!("{:?}", response);

        match response.status() {
            StatusCode::OK => {
                let bankara_open_stages: BankaraOpenStagesResponse = response.json().await?;
                Ok(bankara_open_stages.results)
            }
            status => {
                Err(format!("API request failed with status code: {}", status).into())
            }
        }
    }
}