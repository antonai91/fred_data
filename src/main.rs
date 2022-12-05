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

    // Create the client object
    let mut client = match FredClient::new() {
        Ok(client) => client,
        Err(msg) => {
            println!("{}", msg);
            return
                            },
                                                };
    client.with_key(&fredapi_config.fredapikey);

    // Create the argument builder
    let mut builder = Builder::new();

    // Set the arguments for the builder
    builder
    .observation_start("2000-01-01")
    .units(Units::PCH)
    .frequency(Frequency::A);

    // Make the request and pass in the builder to apply the arguments
    let resp: Response = match client.series_observation("GNPCA", Some(builder)) {
        Ok(resp) => resp,
        Err(msg) => {
            println!("{}", msg);
            return
                    },
        };

    }