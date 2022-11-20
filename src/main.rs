use webeauty::api::WeatherApi;

#[tokio::main]
async fn main() {

    // use your own
    let client = WeatherApi::new("");
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
