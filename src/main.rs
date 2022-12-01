use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use fred_rs::client::FredClient;
use fred_rs::series::observation::{Builder, Units, Frequency, Response};

#[derive(Deserialize, Serialize)]
struct Config {
    fredapikey: String
}

fn main() {
    let f = std::fs::File::open(".apikey/fredapikey.yml").expect("Could not open file.");
    let fredapi_config: Config = serde_yaml::from_reader(f).expect("Could not read values.");
    println!("{:?}", fredapi_config.fredapikey);

    // Create the client object
    let mut client = match FredClient::new() {
        Ok(client) => client,
        Err(msg) => {
            println!("{}", msg);
            return
                            },
                                                };
    client.with_key(&fredapi_config.fredapikey);

        }