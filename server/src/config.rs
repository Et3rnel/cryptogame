use log::error;
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub player_radius: f64,
}

impl Config {
    pub fn load(config_path: &Path) -> Result<Self, Box<dyn Error>> {
        let mut file = File::open(config_path).map_err(|e| {
            error!("Failed to open configuration file: {}", e);
            e
        })?;

        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| {
            error!("Failed to read configuration file: {}", e);
            e
        })?;

        serde_json::from_str(&contents).map_err(|e| {
            error!("Failed to parse configuration JSON: {}", e);
            e.into() // Make sure the error type is consistent with the return type
        })
    }
}
