use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use crate::config::Config;
use crate::storage::{Storage, s3::S3Storage, postgres::PostgresStorage, minio::MinioStorage};

pub struct AppState {
    pub storage: Arc<dyn Storage>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::load().expect("Failed to load config");

    let storage: Arc<dyn Storage> = match config.storage_backend.as_str() {
        "s3" => Arc::new(S3Storage::new(config.s3.bucket, Region::default())),
        "postgres" => Arc::new(PostgresStorage::new(config.postgres.url)),
        "minio" => Arc::new(MinioStorage::new(config.minio.endpoint, config.minio.access_key, config.minio.secret_key, config.minio.bucket)),
        _ => panic!("Unsupported storage backend"),
    };

    let app_state = AppState { storage };

    HttpServer::new(move || {
        App::new()
            .data(app_state.clone())
            .service(api::routes::get_latest)
            .service(api::routes::get_historical)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
