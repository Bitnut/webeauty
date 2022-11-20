use webeauty::api::WeatherApi;
use webeauty::config;

#[tokio::main]
async fn main() {

    let config = config::init_config().expect("init config failed");

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
