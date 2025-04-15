use serde::Deserialize;
use shellexpand;
use std::fs;
use std::io::Error;
use toml;

use crate::config::constants;

#[derive(Deserialize)]
pub struct Config {
    pub ideas_dir: String,
    pub editor: String,
}

pub fn read() -> Result<Config, Error> {
    let binding = shellexpand::tilde(constants::CONFIG_DIR);
    let config_dir = binding.as_ref();
    let config_file_path = format!("{}/{}", &config_dir, constants::CONFIG_FILE);

    // Read config file contents
    let contents = fs::read_to_string(&config_file_path)?;

    let config: Config = toml::from_str(&contents).unwrap();

    Ok(config)
}
