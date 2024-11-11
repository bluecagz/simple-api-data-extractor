use crate::config::CoinMarketCapConfig;
use reqwest::header::{HeaderMap, HeaderValue, InvalidHeaderValue};
use super::models::{FearAndGreedData, HistoricalFearAndGreedData};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("Invalid header value")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),
    #[error("Request error")]
    ReqwestError(#[from] reqwest::Error),
}

pub async fn fetch_latest(config: &CoinMarketCapConfig) -> Result<FearAndGreedData, CustomError> {
    let url = format!("{}{}", config.base_url, config.fear_and_greed_latest_endpoint);

    let mut headers = HeaderMap::new();
    headers.insert("X-CMC_PRO_API_KEY", HeaderValue::from_str(&config.api_key).map_err(CustomError::InvalidHeaderValue)?);

    let client = reqwest::Client::new();
    let response = client.get(&url).headers(headers).send().await;

    match response {
        Ok(resp) => {
            let data = resp.json::<FearAndGreedData>().await.map_err(CustomError::ReqwestError)?;
            Ok(data)
        }
        Err(e) => {
            if e.is_redirect() {
                if let Some(final_stop) = e.url() {
                    println!("redirect loop at {final_stop}");
                }
            }
            Err(CustomError::ReqwestError(e))
        }
    }
}

pub async fn fetch_historical(
    config: &CoinMarketCapConfig,
    start: Option<u32>,
    limit: Option<u32>,
) -> Result<HistoricalFearAndGreedData, CustomError> {
    let url = format!("{}{}", config.base_url, config.fear_and_greed_historical_endpoint);

    let mut headers = HeaderMap::new();
    headers.insert("X-CMC_PRO_API_KEY", HeaderValue::from_str(&config.api_key).map_err(CustomError::InvalidHeaderValue)?);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .headers(headers)
        .query(&[("start", start), ("limit", Some(limit.unwrap_or(50)))])
        .send().await;
    
    match response {
        Ok(resp) => {
            let data = resp.json::<HistoricalFearAndGreedData>().await.map_err(CustomError::ReqwestError)?;
            Ok(data)
        }
        Err(e) => {
            if e.is_redirect() {
                if let Some(final_stop) = e.url() {
                    println!("redirect loop at {final_stop}");
                }
            }
            Err(CustomError::ReqwestError(e))
        }
    }
}
