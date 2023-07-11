use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("ROCKETPOOL", "abi/rocketpool.json")?
        .generate()?
        .write_to_file("src/abi/rocketpool.rs")?;

    Ok(())
}
