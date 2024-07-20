use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CoinbaseCloudApiKey {
    pub name: String,
    #[serde(rename(serialize = "privateKey", deserialize = "privateKey"))]
    pub private_key: String,
}

pub struct CoinbaseCloudApiParser {
    file_name: String,
}

impl CoinbaseCloudApiParser {
    pub fn default() -> Self {
        CoinbaseCloudApiParser { file_name: "private/cdp_api_key.json".parse().unwrap() }
    }

    pub fn new(file_name: String) -> Self {
        CoinbaseCloudApiParser { file_name }
    }

    pub fn parse(self) -> CoinbaseCloudApiKey {
        println!("Reading file {}", self.file_name);
        // dbg!(&args);
        let contents = match fs::read_to_string(self.file_name) {
            Ok(body) => { body }
            Err(error) => {
                println!("Error: {}", error);
                panic!("Problem reading file.");
            }
        };

        match serde_json::from_str(&*contents) {
            Ok(key) => { key }
            Err(error) => {
                println!("Error: {}", error);
                panic!("Problem deserializing api key.");
            }
        }
    }
}