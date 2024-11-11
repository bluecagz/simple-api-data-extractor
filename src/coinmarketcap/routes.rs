use actix_web::{get, web, Responder};
use crate::config::CoinMarketCapConfig;
use super::client;
use super::models::{FearAndGreedData, HistoricalFearAndGreedData, Status, Data};
use std::collections::HashMap;

#[get("/latest")]
async fn get_latest(config: web::Data<CoinMarketCapConfig>) -> impl Responder {
    match client::fetch_latest(&config).await {
        Ok(data) => web::Json(data),
        Err(e) => {
            eprintln!("Failed to fetch latest data: {}", e);
            web::Json(FearAndGreedData {
                status: Status {
                    timestamp: "".to_string(),
                    error_code: "1".to_string(),
                    error_message: Some("Failed to fetch latest data".to_string()),
                    elapsed: 0,
                    credit_count: 0,
                },
                data: Data {
                    value: 0,
                    value_classification: "".to_string(),
                    update_time: "".to_string(),
                },
            })
        }
    }
}

#[get("/historical")]
async fn get_historical(config: web::Data<CoinMarketCapConfig>, query: web::Query<HashMap<String, String>>) -> impl Responder {
    // Parse start and limit parameters from query string
    let start = query.get("start").and_then(|s| s.parse().ok());
    let limit = query.get("limit").and_then(|l| l.parse().ok());

    match client::fetch_historical(&config, start, limit).await {
        Ok(data) => web::Json(data),
        Err(e) => {
            eprintln!("Failed to fetch historical data: {}", e);
            web::Json(HistoricalFearAndGreedData {
                status: Status {
                    timestamp: "".to_string(),
                    error_code: "1".to_string(),
                    error_message: Some("Failed to fetch historical data".to_string()),
                    elapsed: 0,
                    credit_count: 0,
                },
                data: vec![],
            })
        }
    }
}
