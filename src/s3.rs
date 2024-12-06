use rusoto_core::Region;
use rusoto_s3::{S3Client, S3, PutObjectRequest};
use std::sync::Arc;
use crate::config::Config;
use chrono::Utc;

pub async fn store_api_data(config: Arc<Config>) {
    let s3_client = S3Client::new(Region::default());

    // Fetch data from API (example for CoinMarketCap)
    let data = fetch_data_from_api().await;

    // Generate S3 key
    let now = Utc::now();
    let s3_key = format!(
        "api_data/coinmarketcap/{}/{}/{}/{}/{}/latest_{}.json",
        now.year(), now.month(), now.day(), now.hour(), now.minute(), now.timestamp()
    );

    // Store data in S3
    let put_request = PutObjectRequest {
        bucket: config.s3_bucket.clone(),
        key: s3_key,
        body: Some(data.into_bytes().into()),
        ..Default::default()
    };

    s3_client.put_object(put_request).await.expect("Failed to store data in S3");
}

async fn fetch_data_from_api() -> String {
    // Implement the function to fetch data from the API
    // For example, using reqwest to fetch data from CoinMarketCap
    "example data".to_string()
}
