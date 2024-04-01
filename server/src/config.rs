use log::error;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub player_radius: f64,
}

impl Config {
    pub fn load(config_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = match File::open(config_path) {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to open configuration file: {}", e);
                return Err(e.into());
            }
        };

        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents) {
            error!("Failed to read configuration file: {}", e);
            return Err(e.into());
        }

        match serde_json::from_str(&contents) {
            Ok(config) => Ok(config),
            Err(e) => {
                error!("Failed to parse configuration JSON: {}", e);
                return Err(e.into());
            }
        }
    }
}
