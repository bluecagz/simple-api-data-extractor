// src/storage/mod.rs
pub mod s3;
pub mod postgres;
pub mod minio;

use async_trait::async_trait;

#[async_trait]
pub trait Storage {
    async fn fetch_data(&self, key: &str) -> Result<String, Box<dyn std::error::Error>>;
    async fn store_data(&self, key: &str, data: &str) -> Result<(), Box<dyn std::error::Error>>;
}
