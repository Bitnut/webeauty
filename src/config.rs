use std::fs;
use serde::Deserialize;
use super::error::Error;

/// Configuration data
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    /// weather api auth key
    pub api_key: String,
    /// weather api auth id
    pub api_id: Option<String>,
    /// weather api path
    pub api_path: String,
}

pub fn init_config() -> Result<Config, Error> {

    let data = fs::read_to_string("./config.json").expect("error reading config file");

    let config: Config = serde_json::from_str(&data).expect("JSON was not well-formatted");
    println!("{:?}", config);
    Ok(config)
}
