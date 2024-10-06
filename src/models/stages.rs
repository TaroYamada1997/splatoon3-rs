use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BankaraOpenStagesResponse {
    pub results: Vec<Schedule>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Schedule {
    pub start_time: String,
    pub end_time: String,
    pub rule: Rule,
    pub stages: Vec<Stage>,
    pub event: Option<Event>,
    pub is_fest: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    pub key: String,
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stage {
    pub id: u32,
    pub name: String,
    pub image: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub name: String,
    pub desc: String
}