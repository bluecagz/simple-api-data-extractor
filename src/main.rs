use actix_web::{App, HttpServer};
use std::sync::Arc;

mod config;
mod coinmarketcap;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load config once and share across modules
    let config = Arc::new(config::load_config().expect("Failed to load config"));

    HttpServer::new(move || {
        App::new()
            .app_data(config.clone())
            .service(coinmarketcap::routes::get_latest)
            .service(coinmarketcap::routes::get_historical)
            // Future API routes for other services like CoinGecko
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
