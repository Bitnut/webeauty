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
pub struct OpenWeatherClient {
    client: Client,
    api_key: String,
    api_id: String,
    api_path: String
}

#[async_trait::async_trait]
pub trait WeatherClient {
    fn new (config: Config) -> Self;
    async fn get_data (&self) -> Result<String, Error>;
    fn get_origin (&self) -> String;
}

#[async_trait::async_trait]
impl WeatherClient for OpenWeatherClient {

    #[must_use]
    fn new(
        config: Config
    ) -> Self {
        Self {
            client: Client::new(),
            api_key: config.api_key,
            api_id: config.api_id.unwrap_or(String::from("")),
            api_path: config.api_path
        }
    }

    async fn get_data(
        &self,
    ) -> Result<String, Error> {

        let api_key = &self.api_key;
        let api_path = &self.api_path;
        let origin_url = &self.get_origin();
        let base_url = format!("{origin_url}/{api_path}");
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

    fn get_origin(&self) -> String {

        String::from("https://api.openweathermap.org")
    }
}
