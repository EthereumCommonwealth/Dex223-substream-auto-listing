use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("auto_listing", "abis/AutoListing.json")?
        .generate()?
        .write_to_file("src/abi/auto_listings.rs")?;
    Abigen::new("auto_listings_registry", "abis/AutoListingsRegistry.json")?
        .generate()?
        .write_to_file("src/abi/auto_listings_registry.rs")?;
    Abigen::new("pool", "abis/ERC20andERC223.json")?
        .generate()?
        .write_to_file("src/abi/erc20_erc223.rs")?;

    Abigen::new("token_converter", "abis/TokenConverter.json")?
        .generate()?
        .write_to_file("src/abi/token_converter.rs")?;

    Ok(())
}
