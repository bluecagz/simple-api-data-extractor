use tokio::time::{interval, Duration};
use std::sync::Arc;
use crate::config::Config;
use crate::s3::store_api_data;

pub async fn start_scheduler(config: Arc<Config>) {
    let mut interval = interval(Duration::from_secs(600)); // 10 minutes

    loop {
        interval.tick().await;
        // Call the function to store API data in S3
        store_api_data(config.clone()).await;
    }
}
