use std::fs;
use std::io::prelude::*;

use serde::{Serialize, Deserialize};
use toml;

use crate::error::Error;

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigParams
{
    pub token: String,
    pub notify_to: Option<i64>,
}

impl ConfigParams
{
    pub fn fromFile(filename: &std::path::Path) -> Result<Self, Error>
    {
        let mut file = fs::File::open(filename).map_err(
            |_| error!(RuntimeError, format!("Failed to open file {}",
                                              filename.to_string_lossy())))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(
            |_| error!(RuntimeError,
                        format!("Failed to read file {}",
                                filename.to_string_lossy())))?;

        toml::from_str(&contents).map_err(
            |e| error!(RuntimeError,
                        format!("Failed to parse file {}: {}",
                                filename.to_string_lossy(), e)))
    }
}
