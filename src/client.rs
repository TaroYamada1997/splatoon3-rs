use reqwest::Client;

// API クライアントの生成
pub async fn create_client() -> Client {
    reqwest::Client::new()
}
