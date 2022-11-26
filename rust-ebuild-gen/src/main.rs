use serde::{Serialize, Deserialize};
use toml;

#[macro_use]
mod error;
use error::Error;

#[derive(Clone, Debug, Deserialize)]
struct Package
{
    name: String,
    version: String,
}

#[derive(Clone, Deserialize)]
struct LockFile
{
    package: Vec<Package>,
}

fn main() -> Result<(), Error>
{
    let file = String::from_utf8(
        std::fs::read("Cargo.lock").map_err(|_| rterr!("Failed to read lock file"))?)
        .map_err(|_| rterr!("Failed to decode lock file"))?;

    let data: LockFile = toml::from_str(&file)
        .map_err(|_| rterr!("Invalid TOML file"))?;

    for pkg in data.package
    {
        println!("{}-{}", pkg.name, pkg.version);
    }

    Ok(())
}
