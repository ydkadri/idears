use std::io::Error;

use idea::{config, ideas};

fn main() -> Result<(), Error> {
    config::setup::setup()?;

    let config = config::read::read()?;

    ideas::create::create(&config)?;

    Ok(())
}
