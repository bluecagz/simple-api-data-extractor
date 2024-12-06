// src/coinmarketcap/client.rs
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CoinMarketCapResponse {
    // Define the response structure
}

pub struct CoinMarketCapClient {
    client: Client,
    api_key: String,
}

impl CoinMarketCapClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn fetch_data(&self, endpoint: &str) -> Result<CoinMarketCapResponse, Box<dyn std::error::Error>> {
        let url = format!("https://pro-api.coinmarketcap.com/v1/{}", endpoint);
        let response = self.client.get(&url)
            .header("X-CMC_PRO_API_KEY", &self.api_key)
            .send()
            .await?
            .json::<CoinMarketCapResponse>()
            .await?;
        Ok(response)
    }
}
