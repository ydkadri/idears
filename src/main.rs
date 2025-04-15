use std::io::Error;

use idears::{config, ideas};

fn main() -> Result<(), Error> {
    config::setup::setup()?;

    let config = config::read::read()?;

    ideas::create::create(&config)?;

    Ok(())
}
