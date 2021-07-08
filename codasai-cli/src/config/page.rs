use std::path::PathBuf;

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PageConfig {
    pub page_path: PathBuf,
}

impl PageConfig {
    pub fn from_str(config_str: &str) -> Result<Self> {
        Ok(toml::de::from_str(config_str)?)
    }
}
