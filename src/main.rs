use std::io::Error;

use idea::config;

fn main() -> Result<(), Error> {
    config::setup::setup()?;

    Ok(())
}
