use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct CoinMarketCapConfig {
    pub base_url: String,
    pub fear_and_greed_latest_endpoint: String,
    pub fear_and_greed_historical_endpoint: String,
    pub api_key: String,
}

pub fn load_config() -> Result<CoinMarketCapConfig, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string("config.toml")?;
    let config: CoinMarketCapConfig = toml::from_str(&config_content)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        let config = load_config().expect("Failed to load config");
        assert!(!config.base_url.is_empty());
        assert!(!config.api_key.is_empty());
    }
}
