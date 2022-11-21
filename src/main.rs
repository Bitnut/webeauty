mod api;
mod config;
mod error;
use api::WeatherApi;
use config::init_config;

#[tokio::main]
async fn main() {

    let config = init_config().expect("init config failed");

    // use your own
    let client = WeatherApi::new(config);
    let r = client.get_data().await;

    match r {
        Ok(r) => {
            println!("{:?}", r);
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }

}
