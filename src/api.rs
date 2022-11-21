use serde::{Deserialize, Serialize};
#[cfg(feature = "cli")]
use reqwest::{Client, Url};
use crate::config::Config;

use super::error::Error;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WeatherData {
    pub city: (),
    pub cnt: i32,
    pub cod: String,
    pub message: i32,
}

#[derive(Default, Clone)]
pub struct WeatherApi {
    client: Client,
    api_key: String,
    api_id: String,
    api_path: String
}

impl WeatherApi {
    #[must_use]
    pub fn new(
        config: Config
    ) -> Self {
        Self {
            client: Client::new(),
            api_key: config.api_key,
            api_id: config.api_id.unwrap_or(String::from("")),
            api_path: config.api_path
        }
    }

    pub async fn get_data(
        &self,
    ) -> Result<String, Error> {

        let api_key = &self.api_key;
        let api_path = &self.api_path;
        let base_url = format!("https://api.openweathermap.org/{api_path}");
        let url = Url::parse_with_params(
            &base_url,
            &[("lat", "23.1291"), ("lon", "113.2644"), ("appid", &api_key)]
        )?;

        self.client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
            .map_err(Into::into)
    }
}
