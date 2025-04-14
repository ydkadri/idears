mod config;
use std::io::Error;

fn main() -> Result<(), Error> {
    config::setup::setup()?;

    Ok(())
}
