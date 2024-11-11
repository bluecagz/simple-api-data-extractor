use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FearAndGreedData {
    pub status: Status,
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalFearAndGreedData {
    pub status: Status,
    pub data: Vec<Data>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub timestamp: String,
    pub error_code: String,
    pub error_message: Option<String>,
    pub elapsed: u32,
    pub credit_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub value: u32,
    pub value_classification: String,
    pub update_time: String,
}
