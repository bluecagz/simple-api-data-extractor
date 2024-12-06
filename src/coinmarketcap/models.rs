use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FearAndGreedData {
    pub status: Status,
    pub data: DataFearAndGreed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalFearAndGreedData {
    pub status: Status,
    pub data: Vec<DataHistorical>,
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
pub struct DataFearAndGreed {
    pub value: u32,
    pub value_classification: String,
    pub update_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataHistorical {
    pub value: u32,
    pub value_classification: String,
    pub timestap: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalMetricsQuotesLatest {
    pub status: Status,
    pub data: DataGlobalMetricsQuotesLatest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataGlobalMetricsQuotesLatest {
    pub value: u32,
    pub value_classification: String,
    pub update_time: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct DataGlobalMetricsQuotesLatest {
    pub active_cryptocurrencies: u32,
    pub active_exchanges: u32,
    pub active_market_pairs: u32,
    pub btc_dominance: f64,
    pub btc_dominance_24h_percentage_change: f64,
    pub btc_dominance_yesterday: f64,
    pub defi_24h_percentage_change: f64,
    pub defi_market_cap: f64,
    pub defi_volume_24h: f64,
    pub defi_volume_24h_reported: f64,
    pub derivatives_24h_percentage_change: f64,
    pub derivatives_volume_24h: f64,
    pub derivatives_volume_24h_reported: f64,
    pub eth_dominance: f64,
    pub eth_dominance_24h_percentage_change: f64,
    pub eth_dominance_yesterday: f64,
    pub last_updated: String,
    pub quote: Quote,
    pub stablecoin_24h_percentage_change: f64,
    pub stablecoin_market_cap: f64,
    pub stablecoin_volume_24h: f64,
    pub stablecoin_volume_24h_reported: f64,
    pub total_cryptocurrencies: u32,
    pub total_exchanges: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quote {
    pub USD: USD,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct USD {
    pub altcoin_market_cap: f64,
    pub altcoin_volume_24h: f64,
    pub altcoin_volume_24h_reported: f64,
    pub defi_24h_percentage_change: f64,
    pub defi_market_cap: f64,
    pub defi_volume_24h: f64,
    pub defi_volume_24h_reported: f64,
    pub derivatives_24h_percentage_change: f64,
    pub derivatives_volume_24h: f64,
    pub derivatives_volume_24h_reported: f64,
    pub last_updated: String,
    pub stablecoin_24h_percentage_change: f64,
    pub stablecoin_market_cap: f64,
    pub stablecoin_volume_24h: f64,
    pub stablecoin_volume_24h_reported: f64,
    pub total_market_cap: f64,
    pub total_market_cap_yesterday: f64,
    pub total_market_cap_yesterday_percentage_change: f64,
    pub total_volume_24h: f64,
    pub total_volume_24h_reported: f64,
    pub total_volume_24h_yesterday: f64,
    pub total_volume_24h_yesterday_percentage_change: f64,
}
