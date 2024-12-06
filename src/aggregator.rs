// src/aggregator.rs
use crate::storage::Storage;
use crate::coinmarketcap::client::CoinMarketCapClient;
use std::sync::Arc;

pub async fn aggregate_data(storage: Arc<dyn Storage>, cmc_client: &CoinMarketCapClient) -> Result<(), Box<dyn std::error::Error>> {
    let data = cmc_client.fetch_data("global-metrics/quotes/latest").await?;
    let serialized_data = serde_json::to_string(&data)?;

    storage.store_data("global-metrics.json", &serialized_data).await?;
    Ok(())
}
