use serde::{Deserialize, Serialize};
#[cfg(feature = "cli")]
use reqwest::{Client, Url};

use crate::Error;

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
}

impl WeatherApi {
    #[must_use]
    pub fn new(
        api_key: &str,
    ) -> Self {
        Self {
            client: Client::new(),
            api_key: String::from(api_key)
        }
    }

    pub async fn get_data(
        &self
    ) -> Result<String, Error> {

        let api_key = &self.api_key;
        let base_url = String::from("https://api.openweathermap.org/data/2.5/forecast");
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
