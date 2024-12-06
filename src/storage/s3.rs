// src/storage/s3.rs
use rusoto_core::Region;
use rusoto_s3::{S3Client, S3, GetObjectRequest, PutObjectRequest};
use async_trait::async_trait;
use super::Storage;

pub struct S3Storage {
    client: S3Client,
    bucket: String,
}

impl S3Storage {
    pub fn new(bucket: String, region: Region) -> Self {
        Self {
            client: S3Client::new(region),
            bucket,
        }
    }
}

#[async_trait]
impl Storage for S3Storage {
    async fn fetch_data(&self, key: &str) -> Result<String, Box<dyn std::error::Error>> {
        let get_request = GetObjectRequest {
            bucket: self.bucket.clone(),
            key: key.to_string(),
            ..Default::default()
        };

        let result = self.client.get_object(get_request).await?;
        let body = result.body.ok_or("No body in S3 response")?.into_async_read();
        let data: String = serde_json::from_reader(body)?;
        Ok(data)
    }

    async fn store_data(&self, key: &str, data: &str) -> Result<(), Box<dyn std::error::Error>> {
        let put_request = PutObjectRequest {
            bucket: self.bucket.clone(),
            key: key.to_string(),
            body: Some(data.to_string().into_bytes().into()),
            ..Default::default()
        };

        self.client.put_object(put_request).await?;
        Ok(())
    }
}
