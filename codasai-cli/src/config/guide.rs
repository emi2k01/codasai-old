use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GuideConfig {
    pub title: String,
    #[serde(default = "default_pages_path")]
    pub pages_path: PathBuf,
}

impl GuideConfig {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let config_str = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read config at {:?}", path))?;

        toml::de::from_str(&config_str)
            .with_context(|| format!("failed to process config at {:?}", path))
    }
}

fn default_pages_path() -> PathBuf {
    PathBuf::from("_pages/")
}
