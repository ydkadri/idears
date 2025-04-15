use shellexpand;
use std::io::Error;

use crate::config::constants;


pub fn setup() -> Result<(), Error> {
    let binding = shellexpand::tilde(constants::CONFIG_DIR);
    let config_dir = binding.as_ref();

    if !std::path::Path::new(&config_dir).exists() {
        println!("It looks like you haven't set up idea yet. Let's do that now...");

        // Create the config directory
        std::fs::create_dir_all(config_dir)?;

        // Create the config file
        let config_file_path = format!("{}/{}", &config_dir, constants::CONFIG_FILE);
        let mut file = std::fs::File::create(&config_file_path)?;
        std::io::Write::write_all(&mut file, constants::DEFAULT_CONFIG.as_bytes())?;

        println!("Config file created at: {}", config_file_path);
    }

    Ok(())
}
